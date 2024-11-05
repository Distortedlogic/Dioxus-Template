use crate::db::models::{Permission, User};
use dioxus::prelude::*;
use std::str::FromStr;

#[component]
pub fn UserInfo(users: Resource<Result<Vec<User>, ServerFnError>>, idx: Signal<usize>) -> Element {
	let current_idx = idx();

	match users() {
		Some(Ok(users)) => {
			if users.is_empty() {
				return rsx! {
					div { "No users loaded" }
				};
			}
			let current_user = &users[current_idx];
			let first_name = current_user.first_name.clone().unwrap();
			let last_name = current_user.last_name.clone().unwrap();
			let username = current_user.username.clone().unwrap();

			let mut selected_permission = use_signal(|| current_user.permission);

			rsx! {
				div {
					h2 { class: "text-xl font-bold mb-4", "{first_name} {last_name}" }
					div { class: "space-y-2 mb-4",
						select {
							name: "permission",
							oninput: move |ev| {
									selected_permission.set(Permission::from_str(ev.value().as_str()).unwrap());
							},
							option {
								id: "Admin",
								value: "Admin",
								initial_selected: current_user.permission == Permission::Admin,
								selected: current_user.permission == Permission::Admin,
								"Admin"
							}
							option {
								id: "Manager",
								value: "Manager",
								initial_selected: current_user.permission == Permission::Manager,
								selected: current_user.permission == Permission::Manager,
								"Manager"
							}
							option {
								id: "Collaborator",
								value: "Collaborator",
								initial_selected: current_user.permission == Permission::Collaborator,
								selected: current_user.permission == Permission::Collaborator,
								"Collaborator"
							}
							option {
								id: "Viewer",
								value: "Viewer",
								initial_selected: current_user.permission == Permission::Viewer,
								selected: current_user.permission == Permission::Viewer,
								"Viewer"
							}
						}
					}
					div { class: "space-y-2 my-4",
						p { "Username: {username}" }
						p { "Email: {current_user.clone().email}" }
						p { "System Admin: {current_user.clone().system_admin}" }
						p { "Created: {current_user.clone().created_at}" }
						p { "Updated: {current_user.clone().updated_at}" }
					}
				}
			}
		},
		Some(Err(e)) => rsx! {
			div { "Error: {e}" }
		},
		None => rsx! {
			div { "Loading or None..." }
		},
	}
}
