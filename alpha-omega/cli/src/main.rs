use clap::{Parser, Subcommand};
use db::queries::users;
use db::queries::users::Users;
use db::types::public::Permission;
use fake::{faker::name, Fake};
use polars::prelude::ParquetWriter;
use polars::prelude::*;
use std::env;
use std::fs::File;
use std::ops::DerefMut;
use std::path::PathBuf;
use time::OffsetDateTime;
use uuid::Uuid;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
	#[command(subcommand)]
	command: Commands,
}

#[derive(Subcommand)]
enum Commands {
	GenerateUsers {},
}

fn generate_users(count: i32) -> Vec<Users> {
	let mut fake_users = Vec::with_capacity(count as usize);
	let now = OffsetDateTime::now_utc();

	for _ in 0..count {
		let first_name: String = name::en::FirstName().fake();
		let last_name: String = name::en::LastName().fake();
		let email = format!("{}{}@example.com", first_name.to_lowercase(), last_name.to_lowercase());
		let username = format!("{}{}", first_name.to_lowercase(), Uuid::new_v4().simple().to_string().chars().take(8).collect::<String>());

		fake_users.push(Users {
			id: Uuid::new_v4(),
			email,
			username,
			first_name,
			last_name,
			permission: Permission::Viewer,
			system_admin: false,
			created_at: now,
			updated_at: now,
		});
	}
	fake_users
}

fn write_to_parquet(users: &[Users], path: &PathBuf) {
	let mut df = DataFrame::from_iter([
		Column::Series(Series::new("id".into(), users.iter().map(|u| u.id.to_string()).collect::<Vec<_>>())),
		Column::Series(Series::new("email".into(), users.iter().map(|u| u.email.clone()).collect::<Vec<_>>())),
		Column::Series(Series::new("username".into(), users.iter().map(|u| u.username.clone()).collect::<Vec<_>>())),
		Column::Series(Series::new("first_name".into(), users.iter().map(|u| u.first_name.clone()).collect::<Vec<_>>())),
		Column::Series(Series::new("last_name".into(), users.iter().map(|u| u.last_name.clone()).collect::<Vec<_>>())),
		Column::Series(Series::new("permission".into(), users.iter().map(|u| format!("{:?}", u.permission)).collect::<Vec<_>>())),
		Column::Series(Series::new("system_admin".into(), users.iter().map(|u| u.system_admin).collect::<Vec<_>>())),
		Column::Series(Series::new("created_at".into(), users.iter().map(|u| u.created_at.to_string()).collect::<Vec<_>>())),
		Column::Series(Series::new("updated_at".into(), users.iter().map(|u| u.updated_at.to_string()).collect::<Vec<_>>())),
	]);
	let file = File::create(path).unwrap();
	ParquetWriter::new(file).finish(&mut df).unwrap();
}

#[dotenvy::load]
#[tokio::main]
async fn main() {
	let cli = Cli::parse();
	match &cli.command {
		Commands::GenerateUsers {} => {
			let users = generate_users(env::var("USERS_COUNT").unwrap().parse::<i32>().unwrap());
			if let Some(path) = env::var("OUTPUT_PATH").ok().map(PathBuf::from) {
				write_to_parquet(&users, &path);
			}
			if env::var("WRITE_TO_DB").unwrap_or_else(|_| "false".to_string()) == "true" {
				use std::str::FromStr;
				let config = tokio_postgres::Config::from_str(&env::var("DATABASE_URL").unwrap()).unwrap();
				let manager = deadpool_postgres::Manager::new(config, tokio_postgres::NoTls);
				let pool = deadpool_postgres::Pool::builder(manager).build().unwrap();
				let mut conn = pool.get().await.unwrap();
				users::bulk_insert_users().bind(conn.as_mut().deref_mut(), &serde_json::to_value(users).unwrap()).all().await.unwrap();
			}
		},
	}
}
