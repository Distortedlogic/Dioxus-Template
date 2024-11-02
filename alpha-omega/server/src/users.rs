use dioxus::prelude::*;

#[server(UserCount)]
pub async fn user_count() -> Result<i64, ServerFnError> {
	use db::queries::users;
	use db::Pool;
	use std::ops::DerefMut;

	let server_context = server_context();
	let FromContext(pool): FromContext<Pool> = server_context.extract().await?;
	let mut conn = pool.get().await?;
	users::user_count().bind(conn.as_mut().deref_mut()).one().await.map_err(|e| ServerFnError::ServerError(e.to_string()))
}
