use chrono::{DateTime, Utc};
use dioxus::{dioxus_core::AttributeValue, prelude::IntoAttributeValue};
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use uuid::Uuid;

#[cfg(not(target_family = "wasm"))]
use crate::db::schema::users;
#[cfg(not(target_family = "wasm"))]
use diesel::prelude::*;
#[cfg(not(target_family = "wasm"))]
use diesel_derive_enum::DbEnum;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(not(target_family = "wasm"), derive(DbEnum))]
#[cfg_attr(not(target_family = "wasm"), ExistingTypePath = "crate::db::schema::sql_types::Permission")]
pub enum Permission {
	Admin,
	Manager,
	Collaborator,
	Viewer,
}

impl FromStr for Permission {
	type Err = ();
	fn from_str(s: &str) -> Result<Permission, Self::Err> {
		match s {
			"Viewer" => Ok(Permission::Viewer),
			"Manager" => Ok(Permission::Manager),
			"Collaborator" => Ok(Permission::Collaborator),
			"Admin" => Ok(Permission::Admin),
			_ => Err(()),
		}
	}
}

impl IntoAttributeValue for Permission {
	fn into_value(self) -> AttributeValue {
		match self {
			Permission::Viewer => AttributeValue::Text("Viewer".to_string()),
			Permission::Collaborator => AttributeValue::Text("Collaborator".to_string()),
			Permission::Manager => AttributeValue::Text("Manager".to_string()),
			Permission::Admin => AttributeValue::Text("Admin".to_string()),
		}
	}
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(not(target_family = "wasm"), derive(Queryable, Selectable))]
#[cfg_attr(not(target_family = "wasm"), diesel(table_name = users))]
#[cfg_attr(not(target_family = "wasm"), diesel(check_for_backend(diesel::pg::Pg)))]
pub struct User {
	pub id: Uuid,
	pub email: String,
	pub username: Option<String>,
	pub first_name: Option<String>,
	pub last_name: Option<String>,
	pub permission: Permission,
	pub system_admin: bool,
	pub created_at: DateTime<Utc>,
	pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(not(target_family = "wasm"), derive(Insertable))]
#[cfg_attr(not(target_family = "wasm"), diesel(table_name = users))]
pub struct NewUser {
	pub email: String,
	pub username: Option<String>,
	pub first_name: Option<String>,
	pub last_name: Option<String>,
	pub permission: Permission,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(not(target_family = "wasm"), derive(AsChangeset))]
#[cfg_attr(not(target_family = "wasm"), diesel(table_name = users))]
pub struct UpdateUser {
	pub email: Option<String>,
	pub username: Option<String>,
	pub first_name: Option<String>,
	pub last_name: Option<String>,
	pub permission: Option<Permission>,
	pub system_admin: Option<bool>,
}
