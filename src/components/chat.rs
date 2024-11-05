use dioxus::prelude::*;
use futures::StreamExt;
use serde_json::json;
use server_fn::codec::{StreamingText, TextStream};

#[derive(Clone)]
struct ChatMessage {
	content: String,
	is_user: bool,
}

#[server(output = StreamingText)]
async fn stream_chat(message: String) -> Result<TextStream, ServerFnError> {
	use futures::pin_mut;
	use misanthropic::stream::FilterExt;
	use misanthropic::{Client, Model};
	let client = Client::new(std::env::var("ANTHROPIC_API_KEY").unwrap())?;
	let (tx, rx) = futures::channel::mpsc::unbounded();

	tokio::spawn(async move {
		let stream = client
			.stream(json!({
					"model": Model::Sonnet35,
					"messages": [{"role": "user", "content": message}],
					"max_tokens": 4096,
					"temperature": 0.7
			}))
			.await
			.unwrap()
			.filter_rate_limit()
			.text();
		pin_mut!(stream);
		while let Some(Ok(text)) = stream.next().await {
			let _ = tx.unbounded_send(Ok(text.to_string()));
		}
	});

	Ok(TextStream::new(rx))
}

#[component]
pub fn ChatPanel() -> Element {
	let mut messages = use_signal(Vec::<ChatMessage>::new);
	let mut input = use_signal(String::new);

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
					onclick: move |_| async move {
							if input().is_empty() {
									return;
							}
							if let Ok(stream_wrapper) = stream_chat(input.to_string()).await {
									let mut stream = stream_wrapper.into_inner();
									while let Some(Ok(text)) = stream.next().await {
											messages
													.with_mut(|msgs| {
															if let Some(last) = msgs.last_mut() {
																	last.content.push_str(&text);
															}
													});
									}
							}
					},
					"Send"
				}
			}
		}
	}
}
