use dioxus::prelude::*;
use dioxus_core::AttributeValue;
use polars::prelude::*;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub enum JobStatus {
	New,
	Interested,
	Applied,
	Rejected,
	NotInterested,
	Viewed,
}

impl FromStr for JobStatus {
	type Err = ();
	fn from_str(s: &str) -> Result<JobStatus, Self::Err> {
		match s {
			"New" => Ok(JobStatus::New),
			"Interested" => Ok(JobStatus::Interested),
			"Applied" => Ok(JobStatus::Applied),
			"Rejected" => Ok(JobStatus::Rejected),
			"NotInterested" => Ok(JobStatus::NotInterested),
			"Viewed" => Ok(JobStatus::Viewed),
			_ => Err(()),
		}
	}
}

impl IntoAttributeValue for JobStatus {
	fn into_value(self) -> AttributeValue {
		match self {
			JobStatus::New => AttributeValue::Text("New".to_string()),
			JobStatus::Interested => AttributeValue::Text("Interested".to_string()),
			JobStatus::Applied => AttributeValue::Text("Applied".to_string()),
			JobStatus::Rejected => AttributeValue::Text("Rejected".to_string()),
			JobStatus::NotInterested => AttributeValue::Text("NotInterested".to_string()),
			JobStatus::Viewed => AttributeValue::Text("Viewed".to_string()),
		}
	}
}

#[component]
fn Skills(df_signal: Signal<DataFrame>, idx: Signal<usize>) -> Element {
	let df = df_signal();
	let series = df.column("skills").unwrap().list().unwrap().get_as_series(idx()).unwrap();
	rsx! {
		div { class: "flex flex-wrap gap-2 mt-4",
			for skill in series.str().unwrap().into_no_null_iter().collect::<Vec<&str>>() {
				span { class: "bg-gray-100 px-2 py-1 rounded text-sm", "{skill}" }
			}
		}
	}
}

#[component]
pub fn JobInfo(df_signal: Signal<DataFrame>, idx: Signal<usize>) -> Element {
	let df = df_signal();
	let current_idx = idx();
	let series = df.column("skills").unwrap().list().unwrap().get_as_series(current_idx).unwrap();

	let current_status =
		use_memo(move || JobStatus::from_str(df_signal().column("status").unwrap().categorical().unwrap().iter_str().nth(idx()).unwrap().unwrap()).unwrap());
	let title = df.column("title").unwrap().str().unwrap().get(current_idx).unwrap();
	rsx! {
		h2 { class: "text-xl font-bold mb-4", "{title}" }
		div { class: "space-y-2 mb-4",
			for status in ["New", "Interested", "Applied", "Rejected", "NotInterested", "Viewed"] {
				label { class: "flex items-center gap-2",
					input {
						r#type: "radio",
						name: "status",
						checked: current_status() == JobStatus::from_str(status).unwrap(),
						value: status
					}
					"{status}"
				}
			}
		}
		a {
			class: "text-blue-500 hover:underline",
			href: format!(
					"https://www.upwork.com{}",
					df.column("url").unwrap().str().unwrap().get(current_idx).unwrap(),
			),
			target: "_blank",
			"View on Upwork"
		}
		div { class: "space-y-2 my-4",
			match df.column("type").unwrap().str().unwrap().get(idx()) {
					Some(t) => rsx! { p { "Type: {t}" } },
					None => rsx!{}
			},
			match df.column("experience_level").unwrap().str().unwrap().get(idx()) {
					Some(e) => rsx! { p { "Experience: {e}" } },
					None => rsx!{}
			},
			match df.column("budget").unwrap().i64().unwrap().get(idx()) {
					Some(b) => rsx! { p { "Budget: ${b}" } },
					None => rsx!{}
			},
			match df.column("min_rate").unwrap().i64().unwrap().get(idx()) {
					Some(r) => rsx! { p { "Min Rate: ${r}" } },
					None => rsx!{}
			},
			match df.column("max_rate").unwrap().i64().unwrap().get(idx()) {
					Some(r) => rsx! { p { "Max Rate: ${r}" } },
					None => rsx!{}
			}
		}
		div { class: "flex flex-wrap gap-2 mt-4",
			for skill in series.str().unwrap().into_no_null_iter().collect::<Vec<&str>>() {
				span { class: "bg-gray-100 px-2 py-1 rounded text-sm", "{skill}" }
			}
		}
	}
}
