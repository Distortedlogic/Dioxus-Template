use cornucopia::CodegenSettings;
use dotenvy::dotenv;
use postgres::{Client, NoTls};
use std::env;
use std::path::Path;

refinery::embed_migrations!("./migrations");

fn main() {
	dotenv().ok();
	let db_url = env::var("DATABASE_URL").unwrap();
	let queries_path = "queries";
	let file_path = Path::new("src").join("cornucopia.rs");
	let mut client = Client::connect(&db_url, NoTls).unwrap();
	migrations::runner().run(&mut client).unwrap();
	println!("cargo:rerun-if-changed={queries_path}");
	cornucopia::generate_live(&mut client, queries_path, file_path.to_str(), CodegenSettings { is_async: true, derive_ser: true }).unwrap();
}
