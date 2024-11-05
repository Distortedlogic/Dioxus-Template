#![allow(non_snake_case)]
use dioxus::prelude::*;
use dx_starter::components::chat::ChatPanel;
use dx_starter::components::overview_panel::{user_count, OverviewPanel};
use dx_starter::components::user_info::UserInfo;
use dx_starter::db::models::User;

#[component]
fn Index() -> Element {
	let users = use_server_future(get_users)?;
	let count = use_server_future(user_count)?;
	let mut current_idx = use_signal(|| 0);
	let total = match count() {
		Some(Ok(count)) => count as usize,
		_ => 0,
	};
	rsx! {
		document::Link { rel: asset!("assets/tailwind.css") }
		div { class: "h-screen flex flex-col",
			div { class: "flex items-center gap-4 p-4 border-b",
				button {
					class: "px-3 py-1 rounded disabled:opacity-50",
					disabled: current_idx() == 0,
					onclick: move |_| {
							current_idx.set((current_idx()) % total);
					},
					"◀"
				}
				span { "{current_idx + 1}/{total}" }
				button {
					class: "px-3 py-1 rounded disabled:opacity-50",
					disabled: current_idx() >= total,
					onclick: move |_| {
							current_idx.set((current_idx()) % total);
					},
					"▶"
				}
			}
			div { class: "flex flex-1 overflow-hidden",
				div { class: "w-1/3 p-4 border-r overflow-y-auto",
					OverviewPanel {}
					UserInfo { users, idx: current_idx }
				}
				ChatPanel {}
			}
		}
	}
}

#[component]
fn Home() -> Element {
	rsx! {
		div { "just another page" }
	}
}

#[derive(Routable, Clone, PartialEq)]
enum Route {
	#[route("/")]
	Index {},
	#[route("/home")]
	Home {},
}

pub fn App() -> Element {
	rsx! {
		Router::<Route> {
			config: || {
					RouterConfig::default()
							.on_update(|state| {
									(state.current() == Route::Home {})
											.then_some(NavigationTarget::Internal(Route::Index {}))
							})
			}
		}
	}
}
#[server(GetUsers)]
pub async fn get_users() -> Result<Vec<User>, ServerFnError> {
	use diesel::prelude::*;
	use dx_starter::db::schema::users::dsl::*;

	use std::env;
	let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
	let mut pool = PgConnection::establish(&database_url).expect("Error connecting to database");

	users.into_boxed().order(id.asc()).offset(0).limit(25).select(User::as_select()).load(&mut pool).map_err(|e| ServerFnError::ServerError(e.to_string()))
}
