use dioxus::prelude::*;
use server::users::user_count;

#[component]
pub fn OverviewPanel() -> Element {
	let user_count_resource = use_server_future(user_count)?;
	rsx! {
		match user_count_resource() {
				Some(Ok(count)) => rsx! {
						div {
								class: "flex flex-wrap gap-2 mt-4",
								"{count}"
						}
				},
				Some(Err(err)) => rsx! {
						div {
								class: "flex flex-wrap gap-2 mt-4 text-red-500",
								"Error loading job count: {err}"
						}
				},
				None => rsx! {
						div {
								class: "flex flex-wrap gap-2 mt-4",
								"Loading..."
						}
				}
		}
	}
}
