#[cfg(not(target_family = "wasm"))]
pub mod sql_types {
	#[derive(diesel::sql_types::SqlType)]
	#[diesel(postgres_type(name = "permission"))]
	pub struct Permission;
}

#[cfg(not(target_family = "wasm"))]
diesel::table! {
		use diesel::sql_types::*;
		use super::sql_types::Permission;

		users (id) {
				id -> Uuid,
				email -> Text,
				username -> Nullable<Text>,
				first_name -> Nullable<Text>,
				last_name -> Nullable<Text>,
				permission -> Permission,
				system_admin -> Bool,
				created_at -> Timestamptz,
				updated_at -> Timestamptz,
		}
}
