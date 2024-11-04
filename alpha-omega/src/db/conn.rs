#[cfg(not(target_family = "wasm"))]
use diesel::prelude::*;
#[cfg(not(target_family = "wasm"))]
use diesel::r2d2::{self, ConnectionManager};

#[cfg(not(target_family = "wasm"))]
pub type DbPool = diesel::r2d2::Pool<ConnectionManager<PgConnection>>;

#[cfg(not(target_family = "wasm"))]
pub fn establish_connection_pool() -> DbPool {
	use std::env;
	let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
	let manager = ConnectionManager::<PgConnection>::new(database_url);
	r2d2::Pool::builder().build(manager).expect("Failed to create pool")
}
