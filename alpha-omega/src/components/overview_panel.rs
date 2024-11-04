use dioxus::prelude::*;

#[component]
pub fn OverviewPanel() -> Element {
	// #[cfg(feature = "web")]
	// let user_count_resource = use_server_future(user_count)?;

	// #[cfg(not(feature = "web"))]
	let user_count_resource = use_resource(user_count);

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
	use crate::db::conn::DbPool;
	use crate::db::schema::users::dsl::*;
	use diesel::prelude::*;

	let server_context = server_context();
	let FromContext(pool): FromContext<DbPool> = server_context.extract().await?;
	let mut conn = pool.get()?;

	users.count().get_result(&mut conn).map_err(|e| ServerFnError::ServerError(e.to_string()))
}
