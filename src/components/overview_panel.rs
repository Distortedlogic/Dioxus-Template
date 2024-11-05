use dioxus::prelude::*;

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

#[server(UserCount)]
pub async fn user_count() -> Result<i64, ServerFnError> {
	use crate::db::schema::users::dsl::*;
	use diesel::prelude::*;
	use std::env;
	let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
	let mut pool = PgConnection::establish(&database_url).expect("Error connecting to database");
	users.count().get_result(&mut pool).map_err(|e| ServerFnError::ServerError(e.to_string()))
}
