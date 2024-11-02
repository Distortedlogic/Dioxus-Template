use dioxus::prelude::*;
use futures::pin_mut;
use misanthropic::exports::futures::StreamExt;
use misanthropic::prompt::message::Role;
use misanthropic::stream::FilterExt;
use misanthropic::{Client, Model};
use serde_json::json;

#[derive(Clone)]
struct ChatMessage {
	content: String,
	is_user: bool,
}

#[component]
pub fn ChatPanel() -> Element {
	let mut messages = use_signal(Vec::<ChatMessage>::new);
	let mut input = use_signal(String::new);
	let claude = use_signal(|| Client::new(std::env::var("ANTHROPIC_API_KEY").unwrap()).unwrap());

	let chat = use_coroutine(move |mut rx: UnboundedReceiver<String>| async move {
		while let Some(message) = rx.next().await {
			let client = claude();
			let stream = client
				.stream(json!({
						"model": Model::Sonnet35,
						"messages": [{"role": Role::User, "content": message}],
						"max_tokens": 4096,
						"temperature": 0.7
				}))
				.await
				.unwrap()
				.filter_rate_limit()
				.text();
			pin_mut!(stream);
			while let Some(Ok(text)) = stream.next().await {
				messages.with_mut(|msgs| msgs.last_mut().unwrap().content.push_str(text.to_string().as_str()))
			}
		}
	});

	rsx! {
		div { class: "w-1/3 p-4 border-l flex flex-col",
			div { class: "flex-1 overflow-y-auto space-y-4 mb-4",
				for msg in messages() {
					div { class: if msg.is_user { "text-right" } else { "text-left" },
						p { class: "inline-block bg-gray-100 rounded-lg p-3", "{msg.content}" }
					}
				}
			}
			div { class: "flex gap-2",
				input {
					class: "flex-1 px-3 py-2 border rounded",
					value: "{input}",
					placeholder: "Message Claude...",
					onchange: move |evt| input.set(evt.value().clone())
				}
				button {
					class: "px-4 py-2 bg-blue-500 text-white rounded hover:bg-blue-600",
					onclick: move |_| {
							if !input().is_empty() {
									let message = input.to_string();
									messages
											.with_mut(|msgs| {
													msgs.push(ChatMessage {
															content: message.clone(),
															is_user: true,
													});
													msgs.push(ChatMessage {
															content: String::new(),
															is_user: false,
													});
											});
									chat.send(message);
									input.set(String::new());
							}
					},
					"Send"
				}
			}
		}
	}
}
