#![allow(non_snake_case)]
use components::chat_panel::ChatPanel;
use components::job_panels::JobInfo;
// use components::overview_panel::OverviewPanel;
use dioxus::prelude::*;
use polars::prelude::*;

pub fn App() -> Element {
	let df_signal = use_signal(|| LazyFrame::scan_parquet("assets/data/upwork.parquet", Default::default()).unwrap().collect().unwrap());
	let mut current_idx = use_signal(|| 0);
	let df = df_signal();
	let total = df.height();
	rsx! {
		div { class: "h-screen flex flex-col",
			div { class: "flex items-center gap-4 p-4 border-b",
				button {
					class: "px-3 py-1 rounded disabled:opacity-50",
					disabled: current_idx() == 0,
					onclick: move |_| {
							current_idx.set((current_idx() - 1) % total);
					},
					"◀"
				}
				span { "{current_idx + 1}/{total}" }
				button {
					class: "px-3 py-1 rounded disabled:opacity-50",
					disabled: current_idx() >= total - 1,
					onclick: move |_| {
							current_idx.set((current_idx() + 1) % total);
					},
					"▶"
				}
			}
			div { class: "flex flex-1 overflow-hidden",
				div { class: "w-1/3 p-4 border-r overflow-y-auto",
					// OverviewPanel {}
					JobInfo { df_signal, idx: current_idx }
				}
				div { class: "flex-1 p-4 overflow-y-auto",
					textarea {
						class: "w-full h-full p-4 border rounded resize-none",
						readonly: true,
						value: df.column("description").unwrap().str().unwrap().get(current_idx()).unwrap()
					}
				}
				ChatPanel {}
			}
		}
	}
}