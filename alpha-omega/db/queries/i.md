CREATE TABLE users (
id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
email VARCHAR NOT NULL UNIQUE,
username VARCHAR(50),
first_name VARCHAR(50),
last_name VARCHAR(50),
permission permission NOT NULL DEFAULT 'Viewer',
system_admin BOOLEAN NOT NULL DEFAULT false,
created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

write me rust code to generate any number fake users and bulk insert them. u may assume two things, that there is already a user type

#[derive(serde::Serialize, Debug, Clone, PartialEq)]
pub struct Users {
pub id: uuid::Uuid,
pub email: String,
pub username: String,
pub first_name: String,
pub last_name: String,
pub permission: super::super::types::public::Permission,
pub system_admin: bool,
pub created_at: time::OffsetDateTime,
pub updated_at: time::OffsetDateTime,
}

any that there is a function generated for the bulk insert sql
