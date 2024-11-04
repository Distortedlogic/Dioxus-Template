#![allow(non_snake_case)]
use alpha_omega::components::overview_panel::OverviewPanel;
use alpha_omega::components::user_info::UserInfo;
use alpha_omega::db::models::User;
use dioxus::prelude::*;

#[component]
fn Index() -> Element {
	let users = use_server_future(get_users)?;
	let mut current_idx = use_signal(|| 0);
	let total = 500;
	rsx! {
		document::Link { rel: asset!("assets/tailwind.css") }
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
					OverviewPanel {}
					UserInfo { users, idx: current_idx }
				}
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
	use alpha_omega::db::conn::DbPool;
	use alpha_omega::db::schema::users::dsl::*;
	use diesel::prelude::*;

	let server_context = server_context();
	let FromContext(pool): FromContext<DbPool> = server_context.extract().await?;
	let mut conn = pool.get()?;

	users.into_boxed().order(id.asc()).offset(0).limit(25).select(User::as_select()).load(&mut conn).map_err(|e| ServerFnError::ServerError(e.to_string()))
}
