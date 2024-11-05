use clap::{Parser, Subcommand};
use diesel::pg::PgConnection;
use diesel::prelude::*;
use dx_starter::db::models::{NewUser, Permission};
use fake::{faker::name, Fake};
use std::env;
use uuid::Uuid;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
	#[command(subcommand)]
	command: Commands,
}

#[derive(Subcommand)]
enum Commands {
	Users {},
}

fn generate_users(count: i32) -> Vec<NewUser> {
	let mut fake_users = Vec::with_capacity(count as usize);

	for _ in 0..count {
		let first_name: String = name::en::FirstName().fake();
		let last_name: String = name::en::LastName().fake();
		let email = format!("{}{}@example.com", first_name.to_lowercase(), last_name.to_lowercase());
		let username = format!("{}{}", first_name.to_lowercase(), Uuid::new_v4().simple().to_string().chars().take(8).collect::<String>());

		fake_users.push(NewUser {
			email,
			username: Some(username),
			first_name: Some(first_name),
			last_name: Some(last_name),
			permission: Some(Permission::Viewer),
		});
	}
	fake_users
}

#[dotenvy::load]
fn main() {
	let cli = Cli::parse();
	match &cli.command {
		Commands::Users {} => {
			let generated_users = generate_users(env::var("USERS_COUNT").unwrap_or_else(|_| "100".to_string()).parse::<i32>().unwrap());

			use dx_starter::db::schema::users::dsl::*;
			let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
			let mut conn = PgConnection::establish(&database_url).expect("Error connecting to database");

			diesel::insert_into(users).values(&generated_users).execute(&mut conn).expect("Error saving users");
		},
	}
}
