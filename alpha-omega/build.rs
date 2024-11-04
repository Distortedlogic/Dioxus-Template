use diesel::pg::PgConnection;
use diesel::Connection;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use std::env;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

#[dotenvy::load(path = "../.env")]
fn main() {
	println!("cargo:rerun-if-changed=migrations");
	let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
	let mut conn = PgConnection::establish(&database_url).expect("Could not connect to database");
	conn.run_pending_migrations(MIGRATIONS).expect("Could not run migrations");
}
