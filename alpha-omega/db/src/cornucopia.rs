// This file was generated with `cornucopia`. Do not modify.

#[allow(clippy::all, clippy::pedantic)]
#[allow(unused_variables)]
#[allow(unused_imports)]
#[allow(dead_code)]
pub mod types {
	pub mod public {
		#[derive(serde::Serialize, Debug, Clone, Copy, PartialEq, Eq)]
		#[allow(non_camel_case_types)]
		pub enum Permission {
			Admin,
			Manager,
			Collaborator,
			Viewer,
		}
		impl<'a> postgres_types::ToSql for Permission {
			fn to_sql(
				&self, ty: &postgres_types::Type, buf: &mut postgres_types::private::BytesMut,
			) -> Result<postgres_types::IsNull, Box<dyn std::error::Error + Sync + Send>> {
				let s = match *self {
					Permission::Admin => "Admin",
					Permission::Manager => "Manager",
					Permission::Collaborator => "Collaborator",
					Permission::Viewer => "Viewer",
				};
				buf.extend_from_slice(s.as_bytes());
				std::result::Result::Ok(postgres_types::IsNull::No)
			}
			fn accepts(ty: &postgres_types::Type) -> bool {
				if ty.name() != "permission" {
					return false;
				}
				match *ty.kind() {
					postgres_types::Kind::Enum(ref variants) => {
						if variants.len() != 4 {
							return false;
						}
						variants.iter().all(|v| match &**v {
							"Admin" => true,
							"Manager" => true,
							"Collaborator" => true,
							"Viewer" => true,
							_ => false,
						})
					},
					_ => false,
				}
			}
			fn to_sql_checked(
				&self, ty: &postgres_types::Type, out: &mut postgres_types::private::BytesMut,
			) -> Result<postgres_types::IsNull, Box<dyn std::error::Error + Sync + Send>> {
				postgres_types::__to_sql_checked(self, ty, out)
			}
		}
		impl<'a> postgres_types::FromSql<'a> for Permission {
			fn from_sql(ty: &postgres_types::Type, buf: &'a [u8]) -> Result<Permission, Box<dyn std::error::Error + Sync + Send>> {
				match std::str::from_utf8(buf)? {
					"Admin" => Ok(Permission::Admin),
					"Manager" => Ok(Permission::Manager),
					"Collaborator" => Ok(Permission::Collaborator),
					"Viewer" => Ok(Permission::Viewer),
					s => Result::Err(Into::into(format!("invalid variant `{}`", s))),
				}
			}
			fn accepts(ty: &postgres_types::Type) -> bool {
				if ty.name() != "permission" {
					return false;
				}
				match *ty.kind() {
					postgres_types::Kind::Enum(ref variants) => {
						if variants.len() != 4 {
							return false;
						}
						variants.iter().all(|v| match &**v {
							"Admin" => true,
							"Manager" => true,
							"Collaborator" => true,
							"Viewer" => true,
							_ => false,
						})
					},
					_ => false,
				}
			}
		}
	}
}
#[allow(clippy::all, clippy::pedantic)]
#[allow(unused_variables)]
#[allow(unused_imports)]
#[allow(dead_code)]
pub mod queries {
	pub mod users {
		use cornucopia_async::GenericClient;
		use futures;
		use futures::{StreamExt, TryStreamExt};
		#[derive(Debug)]
		pub struct UsersParams<T1: cornucopia_async::StringSql> {
			pub order_by: T1,
			pub offset: i64,
			pub limit: i64,
		}
		#[derive(Debug)]
		pub struct UsersByPermissionParams<T1: cornucopia_async::StringSql> {
			pub permission: super::super::types::public::Permission,
			pub order_by: T1,
			pub offset: i64,
			pub limit: i64,
		}
		#[derive(Clone, Copy, Debug)]
		pub struct UpdateUserPermissionParams {
			pub permission: super::super::types::public::Permission,
			pub id: uuid::Uuid,
		}
		#[derive(Debug)]
		pub struct UpdateUserParams<
			T1: cornucopia_async::StringSql,
			T2: cornucopia_async::StringSql,
			T3: cornucopia_async::StringSql,
			T4: cornucopia_async::StringSql,
		> {
			pub email: T1,
			pub username: T2,
			pub first_name: T3,
			pub last_name: T4,
			pub id: uuid::Uuid,
		}
		#[derive(Debug)]
		pub struct CreateUserParams<
			T1: cornucopia_async::StringSql,
			T2: cornucopia_async::StringSql,
			T3: cornucopia_async::StringSql,
			T4: cornucopia_async::StringSql,
		> {
			pub email: T1,
			pub username: T2,
			pub first_name: T3,
			pub last_name: T4,
			pub permission: super::super::types::public::Permission,
		}
		#[derive(Debug)]
		pub struct SearchUsersParams<T1: cornucopia_async::StringSql, T2: cornucopia_async::StringSql> {
			pub search: T1,
			pub order_by: T2,
			pub offset: i64,
			pub limit: i64,
		}
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
		pub struct UsersBorrowed<'a> {
			pub id: uuid::Uuid,
			pub email: &'a str,
			pub username: &'a str,
			pub first_name: &'a str,
			pub last_name: &'a str,
			pub permission: super::super::types::public::Permission,
			pub system_admin: bool,
			pub created_at: time::OffsetDateTime,
			pub updated_at: time::OffsetDateTime,
		}
		impl<'a> From<UsersBorrowed<'a>> for Users {
			fn from(UsersBorrowed { id, email, username, first_name, last_name, permission, system_admin, created_at, updated_at }: UsersBorrowed<'a>) -> Self {
				Self {
					id,
					email: email.into(),
					username: username.into(),
					first_name: first_name.into(),
					last_name: last_name.into(),
					permission,
					system_admin,
					created_at,
					updated_at,
				}
			}
		}
		pub struct UsersQuery<'a, C: GenericClient, T, const N: usize> {
			client: &'a C,
			params: [&'a (dyn postgres_types::ToSql + Sync); N],
			stmt: &'a mut cornucopia_async::private::Stmt,
			extractor: fn(&tokio_postgres::Row) -> UsersBorrowed,
			mapper: fn(UsersBorrowed) -> T,
		}
		impl<'a, C, T: 'a, const N: usize> UsersQuery<'a, C, T, N>
		where
			C: GenericClient,
		{
			pub fn map<R>(self, mapper: fn(UsersBorrowed) -> R) -> UsersQuery<'a, C, R, N> {
				UsersQuery { client: self.client, params: self.params, stmt: self.stmt, extractor: self.extractor, mapper }
			}
			pub async fn one(self) -> Result<T, tokio_postgres::Error> {
				let stmt = self.stmt.prepare(self.client).await?;
				let row = self.client.query_one(stmt, &self.params).await?;
				Ok((self.mapper)((self.extractor)(&row)))
			}
			pub async fn all(self) -> Result<Vec<T>, tokio_postgres::Error> {
				self.iter().await?.try_collect().await
			}
			pub async fn opt(self) -> Result<Option<T>, tokio_postgres::Error> {
				let stmt = self.stmt.prepare(self.client).await?;
				Ok(self.client.query_opt(stmt, &self.params).await?.map(|row| (self.mapper)((self.extractor)(&row))))
			}
			pub async fn iter(self) -> Result<impl futures::Stream<Item = Result<T, tokio_postgres::Error>> + 'a, tokio_postgres::Error> {
				let stmt = self.stmt.prepare(self.client).await?;
				let it = self
					.client
					.query_raw(stmt, cornucopia_async::private::slice_iter(&self.params))
					.await?
					.map(move |res| res.map(|row| (self.mapper)((self.extractor)(&row))))
					.into_stream();
				Ok(it)
			}
		}
		pub struct I64Query<'a, C: GenericClient, T, const N: usize> {
			client: &'a C,
			params: [&'a (dyn postgres_types::ToSql + Sync); N],
			stmt: &'a mut cornucopia_async::private::Stmt,
			extractor: fn(&tokio_postgres::Row) -> i64,
			mapper: fn(i64) -> T,
		}
		impl<'a, C, T: 'a, const N: usize> I64Query<'a, C, T, N>
		where
			C: GenericClient,
		{
			pub fn map<R>(self, mapper: fn(i64) -> R) -> I64Query<'a, C, R, N> {
				I64Query { client: self.client, params: self.params, stmt: self.stmt, extractor: self.extractor, mapper }
			}
			pub async fn one(self) -> Result<T, tokio_postgres::Error> {
				let stmt = self.stmt.prepare(self.client).await?;
				let row = self.client.query_one(stmt, &self.params).await?;
				Ok((self.mapper)((self.extractor)(&row)))
			}
			pub async fn all(self) -> Result<Vec<T>, tokio_postgres::Error> {
				self.iter().await?.try_collect().await
			}
			pub async fn opt(self) -> Result<Option<T>, tokio_postgres::Error> {
				let stmt = self.stmt.prepare(self.client).await?;
				Ok(self.client.query_opt(stmt, &self.params).await?.map(|row| (self.mapper)((self.extractor)(&row))))
			}
			pub async fn iter(self) -> Result<impl futures::Stream<Item = Result<T, tokio_postgres::Error>> + 'a, tokio_postgres::Error> {
				let stmt = self.stmt.prepare(self.client).await?;
				let it = self
					.client
					.query_raw(stmt, cornucopia_async::private::slice_iter(&self.params))
					.await?
					.map(move |res| res.map(|row| (self.mapper)((self.extractor)(&row))))
					.into_stream();
				Ok(it)
			}
		}
		#[derive(serde::Serialize, Debug, Clone, PartialEq)]
		pub struct UsersByPermission {
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
		pub struct UsersByPermissionBorrowed<'a> {
			pub id: uuid::Uuid,
			pub email: &'a str,
			pub username: &'a str,
			pub first_name: &'a str,
			pub last_name: &'a str,
			pub permission: super::super::types::public::Permission,
			pub system_admin: bool,
			pub created_at: time::OffsetDateTime,
			pub updated_at: time::OffsetDateTime,
		}
		impl<'a> From<UsersByPermissionBorrowed<'a>> for UsersByPermission {
			fn from(
				UsersByPermissionBorrowed { id, email, username, first_name, last_name, permission, system_admin, created_at, updated_at }: UsersByPermissionBorrowed<
					'a,
				>,
			) -> Self {
				Self {
					id,
					email: email.into(),
					username: username.into(),
					first_name: first_name.into(),
					last_name: last_name.into(),
					permission,
					system_admin,
					created_at,
					updated_at,
				}
			}
		}
		pub struct UsersByPermissionQuery<'a, C: GenericClient, T, const N: usize> {
			client: &'a C,
			params: [&'a (dyn postgres_types::ToSql + Sync); N],
			stmt: &'a mut cornucopia_async::private::Stmt,
			extractor: fn(&tokio_postgres::Row) -> UsersByPermissionBorrowed,
			mapper: fn(UsersByPermissionBorrowed) -> T,
		}
		impl<'a, C, T: 'a, const N: usize> UsersByPermissionQuery<'a, C, T, N>
		where
			C: GenericClient,
		{
			pub fn map<R>(self, mapper: fn(UsersByPermissionBorrowed) -> R) -> UsersByPermissionQuery<'a, C, R, N> {
				UsersByPermissionQuery { client: self.client, params: self.params, stmt: self.stmt, extractor: self.extractor, mapper }
			}
			pub async fn one(self) -> Result<T, tokio_postgres::Error> {
				let stmt = self.stmt.prepare(self.client).await?;
				let row = self.client.query_one(stmt, &self.params).await?;
				Ok((self.mapper)((self.extractor)(&row)))
			}
			pub async fn all(self) -> Result<Vec<T>, tokio_postgres::Error> {
				self.iter().await?.try_collect().await
			}
			pub async fn opt(self) -> Result<Option<T>, tokio_postgres::Error> {
				let stmt = self.stmt.prepare(self.client).await?;
				Ok(self.client.query_opt(stmt, &self.params).await?.map(|row| (self.mapper)((self.extractor)(&row))))
			}
			pub async fn iter(self) -> Result<impl futures::Stream<Item = Result<T, tokio_postgres::Error>> + 'a, tokio_postgres::Error> {
				let stmt = self.stmt.prepare(self.client).await?;
				let it = self
					.client
					.query_raw(stmt, cornucopia_async::private::slice_iter(&self.params))
					.await?
					.map(move |res| res.map(|row| (self.mapper)((self.extractor)(&row))))
					.into_stream();
				Ok(it)
			}
		}
		#[derive(serde::Serialize, Debug, Clone, PartialEq)]
		pub struct UsersByEmail {
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
		pub struct UsersByEmailBorrowed<'a> {
			pub id: uuid::Uuid,
			pub email: &'a str,
			pub username: &'a str,
			pub first_name: &'a str,
			pub last_name: &'a str,
			pub permission: super::super::types::public::Permission,
			pub system_admin: bool,
			pub created_at: time::OffsetDateTime,
			pub updated_at: time::OffsetDateTime,
		}
		impl<'a> From<UsersByEmailBorrowed<'a>> for UsersByEmail {
			fn from(
				UsersByEmailBorrowed { id, email, username, first_name, last_name, permission, system_admin, created_at, updated_at }: UsersByEmailBorrowed<'a>,
			) -> Self {
				Self {
					id,
					email: email.into(),
					username: username.into(),
					first_name: first_name.into(),
					last_name: last_name.into(),
					permission,
					system_admin,
					created_at,
					updated_at,
				}
			}
		}
		pub struct UsersByEmailQuery<'a, C: GenericClient, T, const N: usize> {
			client: &'a C,
			params: [&'a (dyn postgres_types::ToSql + Sync); N],
			stmt: &'a mut cornucopia_async::private::Stmt,
			extractor: fn(&tokio_postgres::Row) -> UsersByEmailBorrowed,
			mapper: fn(UsersByEmailBorrowed) -> T,
		}
		impl<'a, C, T: 'a, const N: usize> UsersByEmailQuery<'a, C, T, N>
		where
			C: GenericClient,
		{
			pub fn map<R>(self, mapper: fn(UsersByEmailBorrowed) -> R) -> UsersByEmailQuery<'a, C, R, N> {
				UsersByEmailQuery { client: self.client, params: self.params, stmt: self.stmt, extractor: self.extractor, mapper }
			}
			pub async fn one(self) -> Result<T, tokio_postgres::Error> {
				let stmt = self.stmt.prepare(self.client).await?;
				let row = self.client.query_one(stmt, &self.params).await?;
				Ok((self.mapper)((self.extractor)(&row)))
			}
			pub async fn all(self) -> Result<Vec<T>, tokio_postgres::Error> {
				self.iter().await?.try_collect().await
			}
			pub async fn opt(self) -> Result<Option<T>, tokio_postgres::Error> {
				let stmt = self.stmt.prepare(self.client).await?;
				Ok(self.client.query_opt(stmt, &self.params).await?.map(|row| (self.mapper)((self.extractor)(&row))))
			}
			pub async fn iter(self) -> Result<impl futures::Stream<Item = Result<T, tokio_postgres::Error>> + 'a, tokio_postgres::Error> {
				let stmt = self.stmt.prepare(self.client).await?;
				let it = self
					.client
					.query_raw(stmt, cornucopia_async::private::slice_iter(&self.params))
					.await?
					.map(move |res| res.map(|row| (self.mapper)((self.extractor)(&row))))
					.into_stream();
				Ok(it)
			}
		}
		#[derive(serde::Serialize, Debug, Clone, PartialEq)]
		pub struct UsersByUsername {
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
		pub struct UsersByUsernameBorrowed<'a> {
			pub id: uuid::Uuid,
			pub email: &'a str,
			pub username: &'a str,
			pub first_name: &'a str,
			pub last_name: &'a str,
			pub permission: super::super::types::public::Permission,
			pub system_admin: bool,
			pub created_at: time::OffsetDateTime,
			pub updated_at: time::OffsetDateTime,
		}
		impl<'a> From<UsersByUsernameBorrowed<'a>> for UsersByUsername {
			fn from(
				UsersByUsernameBorrowed { id, email, username, first_name, last_name, permission, system_admin, created_at, updated_at }: UsersByUsernameBorrowed<'a>,
			) -> Self {
				Self {
					id,
					email: email.into(),
					username: username.into(),
					first_name: first_name.into(),
					last_name: last_name.into(),
					permission,
					system_admin,
					created_at,
					updated_at,
				}
			}
		}
		pub struct UsersByUsernameQuery<'a, C: GenericClient, T, const N: usize> {
			client: &'a C,
			params: [&'a (dyn postgres_types::ToSql + Sync); N],
			stmt: &'a mut cornucopia_async::private::Stmt,
			extractor: fn(&tokio_postgres::Row) -> UsersByUsernameBorrowed,
			mapper: fn(UsersByUsernameBorrowed) -> T,
		}
		impl<'a, C, T: 'a, const N: usize> UsersByUsernameQuery<'a, C, T, N>
		where
			C: GenericClient,
		{
			pub fn map<R>(self, mapper: fn(UsersByUsernameBorrowed) -> R) -> UsersByUsernameQuery<'a, C, R, N> {
				UsersByUsernameQuery { client: self.client, params: self.params, stmt: self.stmt, extractor: self.extractor, mapper }
			}
			pub async fn one(self) -> Result<T, tokio_postgres::Error> {
				let stmt = self.stmt.prepare(self.client).await?;
				let row = self.client.query_one(stmt, &self.params).await?;
				Ok((self.mapper)((self.extractor)(&row)))
			}
			pub async fn all(self) -> Result<Vec<T>, tokio_postgres::Error> {
				self.iter().await?.try_collect().await
			}
			pub async fn opt(self) -> Result<Option<T>, tokio_postgres::Error> {
				let stmt = self.stmt.prepare(self.client).await?;
				Ok(self.client.query_opt(stmt, &self.params).await?.map(|row| (self.mapper)((self.extractor)(&row))))
			}
			pub async fn iter(self) -> Result<impl futures::Stream<Item = Result<T, tokio_postgres::Error>> + 'a, tokio_postgres::Error> {
				let stmt = self.stmt.prepare(self.client).await?;
				let it = self
					.client
					.query_raw(stmt, cornucopia_async::private::slice_iter(&self.params))
					.await?
					.map(move |res| res.map(|row| (self.mapper)((self.extractor)(&row))))
					.into_stream();
				Ok(it)
			}
		}
		#[derive(serde::Serialize, Debug, Clone, PartialEq)]
		pub struct UpdateUserPermission {
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
		pub struct UpdateUserPermissionBorrowed<'a> {
			pub id: uuid::Uuid,
			pub email: &'a str,
			pub username: &'a str,
			pub first_name: &'a str,
			pub last_name: &'a str,
			pub permission: super::super::types::public::Permission,
			pub system_admin: bool,
			pub created_at: time::OffsetDateTime,
			pub updated_at: time::OffsetDateTime,
		}
		impl<'a> From<UpdateUserPermissionBorrowed<'a>> for UpdateUserPermission {
			fn from(
				UpdateUserPermissionBorrowed { id,email,username,first_name,last_name,permission,system_admin,created_at,updated_at,}: UpdateUserPermissionBorrowed<'a>,
			) -> Self {
				Self {
					id,
					email: email.into(),
					username: username.into(),
					first_name: first_name.into(),
					last_name: last_name.into(),
					permission,
					system_admin,
					created_at,
					updated_at,
				}
			}
		}
		pub struct UpdateUserPermissionQuery<'a, C: GenericClient, T, const N: usize> {
			client: &'a C,
			params: [&'a (dyn postgres_types::ToSql + Sync); N],
			stmt: &'a mut cornucopia_async::private::Stmt,
			extractor: fn(&tokio_postgres::Row) -> UpdateUserPermissionBorrowed,
			mapper: fn(UpdateUserPermissionBorrowed) -> T,
		}
		impl<'a, C, T: 'a, const N: usize> UpdateUserPermissionQuery<'a, C, T, N>
		where
			C: GenericClient,
		{
			pub fn map<R>(self, mapper: fn(UpdateUserPermissionBorrowed) -> R) -> UpdateUserPermissionQuery<'a, C, R, N> {
				UpdateUserPermissionQuery { client: self.client, params: self.params, stmt: self.stmt, extractor: self.extractor, mapper }
			}
			pub async fn one(self) -> Result<T, tokio_postgres::Error> {
				let stmt = self.stmt.prepare(self.client).await?;
				let row = self.client.query_one(stmt, &self.params).await?;
				Ok((self.mapper)((self.extractor)(&row)))
			}
			pub async fn all(self) -> Result<Vec<T>, tokio_postgres::Error> {
				self.iter().await?.try_collect().await
			}
			pub async fn opt(self) -> Result<Option<T>, tokio_postgres::Error> {
				let stmt = self.stmt.prepare(self.client).await?;
				Ok(self.client.query_opt(stmt, &self.params).await?.map(|row| (self.mapper)((self.extractor)(&row))))
			}
			pub async fn iter(self) -> Result<impl futures::Stream<Item = Result<T, tokio_postgres::Error>> + 'a, tokio_postgres::Error> {
				let stmt = self.stmt.prepare(self.client).await?;
				let it = self
					.client
					.query_raw(stmt, cornucopia_async::private::slice_iter(&self.params))
					.await?
					.map(move |res| res.map(|row| (self.mapper)((self.extractor)(&row))))
					.into_stream();
				Ok(it)
			}
		}
		#[derive(serde::Serialize, Debug, Clone, PartialEq)]
		pub struct UpdateUser {
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
		pub struct UpdateUserBorrowed<'a> {
			pub id: uuid::Uuid,
			pub email: &'a str,
			pub username: &'a str,
			pub first_name: &'a str,
			pub last_name: &'a str,
			pub permission: super::super::types::public::Permission,
			pub system_admin: bool,
			pub created_at: time::OffsetDateTime,
			pub updated_at: time::OffsetDateTime,
		}
		impl<'a> From<UpdateUserBorrowed<'a>> for UpdateUser {
			fn from(
				UpdateUserBorrowed { id, email, username, first_name, last_name, permission, system_admin, created_at, updated_at }: UpdateUserBorrowed<'a>,
			) -> Self {
				Self {
					id,
					email: email.into(),
					username: username.into(),
					first_name: first_name.into(),
					last_name: last_name.into(),
					permission,
					system_admin,
					created_at,
					updated_at,
				}
			}
		}
		pub struct UpdateUserQuery<'a, C: GenericClient, T, const N: usize> {
			client: &'a C,
			params: [&'a (dyn postgres_types::ToSql + Sync); N],
			stmt: &'a mut cornucopia_async::private::Stmt,
			extractor: fn(&tokio_postgres::Row) -> UpdateUserBorrowed,
			mapper: fn(UpdateUserBorrowed) -> T,
		}
		impl<'a, C, T: 'a, const N: usize> UpdateUserQuery<'a, C, T, N>
		where
			C: GenericClient,
		{
			pub fn map<R>(self, mapper: fn(UpdateUserBorrowed) -> R) -> UpdateUserQuery<'a, C, R, N> {
				UpdateUserQuery { client: self.client, params: self.params, stmt: self.stmt, extractor: self.extractor, mapper }
			}
			pub async fn one(self) -> Result<T, tokio_postgres::Error> {
				let stmt = self.stmt.prepare(self.client).await?;
				let row = self.client.query_one(stmt, &self.params).await?;
				Ok((self.mapper)((self.extractor)(&row)))
			}
			pub async fn all(self) -> Result<Vec<T>, tokio_postgres::Error> {
				self.iter().await?.try_collect().await
			}
			pub async fn opt(self) -> Result<Option<T>, tokio_postgres::Error> {
				let stmt = self.stmt.prepare(self.client).await?;
				Ok(self.client.query_opt(stmt, &self.params).await?.map(|row| (self.mapper)((self.extractor)(&row))))
			}
			pub async fn iter(self) -> Result<impl futures::Stream<Item = Result<T, tokio_postgres::Error>> + 'a, tokio_postgres::Error> {
				let stmt = self.stmt.prepare(self.client).await?;
				let it = self
					.client
					.query_raw(stmt, cornucopia_async::private::slice_iter(&self.params))
					.await?
					.map(move |res| res.map(|row| (self.mapper)((self.extractor)(&row))))
					.into_stream();
				Ok(it)
			}
		}
		#[derive(serde::Serialize, Debug, Clone, PartialEq)]
		pub struct CreateUser {
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
		pub struct CreateUserBorrowed<'a> {
			pub id: uuid::Uuid,
			pub email: &'a str,
			pub username: &'a str,
			pub first_name: &'a str,
			pub last_name: &'a str,
			pub permission: super::super::types::public::Permission,
			pub system_admin: bool,
			pub created_at: time::OffsetDateTime,
			pub updated_at: time::OffsetDateTime,
		}
		impl<'a> From<CreateUserBorrowed<'a>> for CreateUser {
			fn from(
				CreateUserBorrowed { id, email, username, first_name, last_name, permission, system_admin, created_at, updated_at }: CreateUserBorrowed<'a>,
			) -> Self {
				Self {
					id,
					email: email.into(),
					username: username.into(),
					first_name: first_name.into(),
					last_name: last_name.into(),
					permission,
					system_admin,
					created_at,
					updated_at,
				}
			}
		}
		pub struct CreateUserQuery<'a, C: GenericClient, T, const N: usize> {
			client: &'a C,
			params: [&'a (dyn postgres_types::ToSql + Sync); N],
			stmt: &'a mut cornucopia_async::private::Stmt,
			extractor: fn(&tokio_postgres::Row) -> CreateUserBorrowed,
			mapper: fn(CreateUserBorrowed) -> T,
		}
		impl<'a, C, T: 'a, const N: usize> CreateUserQuery<'a, C, T, N>
		where
			C: GenericClient,
		{
			pub fn map<R>(self, mapper: fn(CreateUserBorrowed) -> R) -> CreateUserQuery<'a, C, R, N> {
				CreateUserQuery { client: self.client, params: self.params, stmt: self.stmt, extractor: self.extractor, mapper }
			}
			pub async fn one(self) -> Result<T, tokio_postgres::Error> {
				let stmt = self.stmt.prepare(self.client).await?;
				let row = self.client.query_one(stmt, &self.params).await?;
				Ok((self.mapper)((self.extractor)(&row)))
			}
			pub async fn all(self) -> Result<Vec<T>, tokio_postgres::Error> {
				self.iter().await?.try_collect().await
			}
			pub async fn opt(self) -> Result<Option<T>, tokio_postgres::Error> {
				let stmt = self.stmt.prepare(self.client).await?;
				Ok(self.client.query_opt(stmt, &self.params).await?.map(|row| (self.mapper)((self.extractor)(&row))))
			}
			pub async fn iter(self) -> Result<impl futures::Stream<Item = Result<T, tokio_postgres::Error>> + 'a, tokio_postgres::Error> {
				let stmt = self.stmt.prepare(self.client).await?;
				let it = self
					.client
					.query_raw(stmt, cornucopia_async::private::slice_iter(&self.params))
					.await?
					.map(move |res| res.map(|row| (self.mapper)((self.extractor)(&row))))
					.into_stream();
				Ok(it)
			}
		}
		#[derive(serde::Serialize, Debug, Clone, PartialEq)]
		pub struct BulkInsertUsers {
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
		pub struct BulkInsertUsersBorrowed<'a> {
			pub id: uuid::Uuid,
			pub email: &'a str,
			pub username: &'a str,
			pub first_name: &'a str,
			pub last_name: &'a str,
			pub permission: super::super::types::public::Permission,
			pub system_admin: bool,
			pub created_at: time::OffsetDateTime,
			pub updated_at: time::OffsetDateTime,
		}
		impl<'a> From<BulkInsertUsersBorrowed<'a>> for BulkInsertUsers {
			fn from(
				BulkInsertUsersBorrowed { id, email, username, first_name, last_name, permission, system_admin, created_at, updated_at }: BulkInsertUsersBorrowed<'a>,
			) -> Self {
				Self {
					id,
					email: email.into(),
					username: username.into(),
					first_name: first_name.into(),
					last_name: last_name.into(),
					permission,
					system_admin,
					created_at,
					updated_at,
				}
			}
		}
		pub struct BulkInsertUsersQuery<'a, C: GenericClient, T, const N: usize> {
			client: &'a C,
			params: [&'a (dyn postgres_types::ToSql + Sync); N],
			stmt: &'a mut cornucopia_async::private::Stmt,
			extractor: fn(&tokio_postgres::Row) -> BulkInsertUsersBorrowed,
			mapper: fn(BulkInsertUsersBorrowed) -> T,
		}
		impl<'a, C, T: 'a, const N: usize> BulkInsertUsersQuery<'a, C, T, N>
		where
			C: GenericClient,
		{
			pub fn map<R>(self, mapper: fn(BulkInsertUsersBorrowed) -> R) -> BulkInsertUsersQuery<'a, C, R, N> {
				BulkInsertUsersQuery { client: self.client, params: self.params, stmt: self.stmt, extractor: self.extractor, mapper }
			}
			pub async fn one(self) -> Result<T, tokio_postgres::Error> {
				let stmt = self.stmt.prepare(self.client).await?;
				let row = self.client.query_one(stmt, &self.params).await?;
				Ok((self.mapper)((self.extractor)(&row)))
			}
			pub async fn all(self) -> Result<Vec<T>, tokio_postgres::Error> {
				self.iter().await?.try_collect().await
			}
			pub async fn opt(self) -> Result<Option<T>, tokio_postgres::Error> {
				let stmt = self.stmt.prepare(self.client).await?;
				Ok(self.client.query_opt(stmt, &self.params).await?.map(|row| (self.mapper)((self.extractor)(&row))))
			}
			pub async fn iter(self) -> Result<impl futures::Stream<Item = Result<T, tokio_postgres::Error>> + 'a, tokio_postgres::Error> {
				let stmt = self.stmt.prepare(self.client).await?;
				let it = self
					.client
					.query_raw(stmt, cornucopia_async::private::slice_iter(&self.params))
					.await?
					.map(move |res| res.map(|row| (self.mapper)((self.extractor)(&row))))
					.into_stream();
				Ok(it)
			}
		}
		#[derive(serde::Serialize, Debug, Clone, PartialEq)]
		pub struct DeleteUser {
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
		pub struct DeleteUserBorrowed<'a> {
			pub id: uuid::Uuid,
			pub email: &'a str,
			pub username: &'a str,
			pub first_name: &'a str,
			pub last_name: &'a str,
			pub permission: super::super::types::public::Permission,
			pub system_admin: bool,
			pub created_at: time::OffsetDateTime,
			pub updated_at: time::OffsetDateTime,
		}
		impl<'a> From<DeleteUserBorrowed<'a>> for DeleteUser {
			fn from(
				DeleteUserBorrowed { id, email, username, first_name, last_name, permission, system_admin, created_at, updated_at }: DeleteUserBorrowed<'a>,
			) -> Self {
				Self {
					id,
					email: email.into(),
					username: username.into(),
					first_name: first_name.into(),
					last_name: last_name.into(),
					permission,
					system_admin,
					created_at,
					updated_at,
				}
			}
		}
		pub struct DeleteUserQuery<'a, C: GenericClient, T, const N: usize> {
			client: &'a C,
			params: [&'a (dyn postgres_types::ToSql + Sync); N],
			stmt: &'a mut cornucopia_async::private::Stmt,
			extractor: fn(&tokio_postgres::Row) -> DeleteUserBorrowed,
			mapper: fn(DeleteUserBorrowed) -> T,
		}
		impl<'a, C, T: 'a, const N: usize> DeleteUserQuery<'a, C, T, N>
		where
			C: GenericClient,
		{
			pub fn map<R>(self, mapper: fn(DeleteUserBorrowed) -> R) -> DeleteUserQuery<'a, C, R, N> {
				DeleteUserQuery { client: self.client, params: self.params, stmt: self.stmt, extractor: self.extractor, mapper }
			}
			pub async fn one(self) -> Result<T, tokio_postgres::Error> {
				let stmt = self.stmt.prepare(self.client).await?;
				let row = self.client.query_one(stmt, &self.params).await?;
				Ok((self.mapper)((self.extractor)(&row)))
			}
			pub async fn all(self) -> Result<Vec<T>, tokio_postgres::Error> {
				self.iter().await?.try_collect().await
			}
			pub async fn opt(self) -> Result<Option<T>, tokio_postgres::Error> {
				let stmt = self.stmt.prepare(self.client).await?;
				Ok(self.client.query_opt(stmt, &self.params).await?.map(|row| (self.mapper)((self.extractor)(&row))))
			}
			pub async fn iter(self) -> Result<impl futures::Stream<Item = Result<T, tokio_postgres::Error>> + 'a, tokio_postgres::Error> {
				let stmt = self.stmt.prepare(self.client).await?;
				let it = self
					.client
					.query_raw(stmt, cornucopia_async::private::slice_iter(&self.params))
					.await?
					.map(move |res| res.map(|row| (self.mapper)((self.extractor)(&row))))
					.into_stream();
				Ok(it)
			}
		}
		#[derive(serde::Serialize, Debug, Clone, PartialEq)]
		pub struct SearchUsers {
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
		pub struct SearchUsersBorrowed<'a> {
			pub id: uuid::Uuid,
			pub email: &'a str,
			pub username: &'a str,
			pub first_name: &'a str,
			pub last_name: &'a str,
			pub permission: super::super::types::public::Permission,
			pub system_admin: bool,
			pub created_at: time::OffsetDateTime,
			pub updated_at: time::OffsetDateTime,
		}
		impl<'a> From<SearchUsersBorrowed<'a>> for SearchUsers {
			fn from(
				SearchUsersBorrowed { id, email, username, first_name, last_name, permission, system_admin, created_at, updated_at }: SearchUsersBorrowed<'a>,
			) -> Self {
				Self {
					id,
					email: email.into(),
					username: username.into(),
					first_name: first_name.into(),
					last_name: last_name.into(),
					permission,
					system_admin,
					created_at,
					updated_at,
				}
			}
		}
		pub struct SearchUsersQuery<'a, C: GenericClient, T, const N: usize> {
			client: &'a C,
			params: [&'a (dyn postgres_types::ToSql + Sync); N],
			stmt: &'a mut cornucopia_async::private::Stmt,
			extractor: fn(&tokio_postgres::Row) -> SearchUsersBorrowed,
			mapper: fn(SearchUsersBorrowed) -> T,
		}
		impl<'a, C, T: 'a, const N: usize> SearchUsersQuery<'a, C, T, N>
		where
			C: GenericClient,
		{
			pub fn map<R>(self, mapper: fn(SearchUsersBorrowed) -> R) -> SearchUsersQuery<'a, C, R, N> {
				SearchUsersQuery { client: self.client, params: self.params, stmt: self.stmt, extractor: self.extractor, mapper }
			}
			pub async fn one(self) -> Result<T, tokio_postgres::Error> {
				let stmt = self.stmt.prepare(self.client).await?;
				let row = self.client.query_one(stmt, &self.params).await?;
				Ok((self.mapper)((self.extractor)(&row)))
			}
			pub async fn all(self) -> Result<Vec<T>, tokio_postgres::Error> {
				self.iter().await?.try_collect().await
			}
			pub async fn opt(self) -> Result<Option<T>, tokio_postgres::Error> {
				let stmt = self.stmt.prepare(self.client).await?;
				Ok(self.client.query_opt(stmt, &self.params).await?.map(|row| (self.mapper)((self.extractor)(&row))))
			}
			pub async fn iter(self) -> Result<impl futures::Stream<Item = Result<T, tokio_postgres::Error>> + 'a, tokio_postgres::Error> {
				let stmt = self.stmt.prepare(self.client).await?;
				let it = self
					.client
					.query_raw(stmt, cornucopia_async::private::slice_iter(&self.params))
					.await?
					.map(move |res| res.map(|row| (self.mapper)((self.extractor)(&row))))
					.into_stream();
				Ok(it)
			}
		}
		pub fn users() -> UsersStmt {
			UsersStmt(cornucopia_async::private::Stmt::new(
				"SELECT *
FROM users
ORDER BY $1 DESC
OFFSET $2
LIMIT $3",
			))
		}
		pub struct UsersStmt(cornucopia_async::private::Stmt);
		impl UsersStmt {
			pub fn bind<'a, C: GenericClient, T1: cornucopia_async::StringSql>(
				&'a mut self, client: &'a C, order_by: &'a T1, offset: &'a i64, limit: &'a i64,
			) -> UsersQuery<'a, C, Users, 3> {
				UsersQuery {
					client,
					params: [order_by, offset, limit],
					stmt: &mut self.0,
					extractor: |row| UsersBorrowed {
						id: row.get(0),
						email: row.get(1),
						username: row.get(2),
						first_name: row.get(3),
						last_name: row.get(4),
						permission: row.get(5),
						system_admin: row.get(6),
						created_at: row.get(7),
						updated_at: row.get(8),
					},
					mapper: |it| <Users>::from(it),
				}
			}
		}
		impl<'a, C: GenericClient, T1: cornucopia_async::StringSql> cornucopia_async::Params<'a, UsersParams<T1>, UsersQuery<'a, C, Users, 3>, C> for UsersStmt {
			fn params(&'a mut self, client: &'a C, params: &'a UsersParams<T1>) -> UsersQuery<'a, C, Users, 3> {
				self.bind(client, &params.order_by, &params.offset, &params.limit)
			}
		}
		pub fn user_count() -> UserCountStmt {
			UserCountStmt(cornucopia_async::private::Stmt::new(
				"SELECT COUNT(*)
FROM users",
			))
		}
		pub struct UserCountStmt(cornucopia_async::private::Stmt);
		impl UserCountStmt {
			pub fn bind<'a, C: GenericClient>(&'a mut self, client: &'a C) -> I64Query<'a, C, i64, 0> {
				I64Query { client, params: [], stmt: &mut self.0, extractor: |row| row.get(0), mapper: |it| it }
			}
		}
		pub fn users_by_permission() -> UsersByPermissionStmt {
			UsersByPermissionStmt(cornucopia_async::private::Stmt::new(
				"SELECT *
FROM users
WHERE permission = $1::permission
ORDER BY $2 DESC
OFFSET $3
LIMIT $4",
			))
		}
		pub struct UsersByPermissionStmt(cornucopia_async::private::Stmt);
		impl UsersByPermissionStmt {
			pub fn bind<'a, C: GenericClient, T1: cornucopia_async::StringSql>(
				&'a mut self, client: &'a C, permission: &'a super::super::types::public::Permission, order_by: &'a T1, offset: &'a i64, limit: &'a i64,
			) -> UsersByPermissionQuery<'a, C, UsersByPermission, 4> {
				UsersByPermissionQuery {
					client,
					params: [permission, order_by, offset, limit],
					stmt: &mut self.0,
					extractor: |row| UsersByPermissionBorrowed {
						id: row.get(0),
						email: row.get(1),
						username: row.get(2),
						first_name: row.get(3),
						last_name: row.get(4),
						permission: row.get(5),
						system_admin: row.get(6),
						created_at: row.get(7),
						updated_at: row.get(8),
					},
					mapper: |it| <UsersByPermission>::from(it),
				}
			}
		}
		impl<'a, C: GenericClient, T1: cornucopia_async::StringSql>
			cornucopia_async::Params<'a, UsersByPermissionParams<T1>, UsersByPermissionQuery<'a, C, UsersByPermission, 4>, C> for UsersByPermissionStmt
		{
			fn params(&'a mut self, client: &'a C, params: &'a UsersByPermissionParams<T1>) -> UsersByPermissionQuery<'a, C, UsersByPermission, 4> {
				self.bind(client, &params.permission, &params.order_by, &params.offset, &params.limit)
			}
		}
		pub fn users_by_email() -> UsersByEmailStmt {
			UsersByEmailStmt(cornucopia_async::private::Stmt::new(
				"SELECT *
FROM users
WHERE email = $1
LIMIT 1",
			))
		}
		pub struct UsersByEmailStmt(cornucopia_async::private::Stmt);
		impl UsersByEmailStmt {
			pub fn bind<'a, C: GenericClient, T1: cornucopia_async::StringSql>(
				&'a mut self, client: &'a C, email: &'a T1,
			) -> UsersByEmailQuery<'a, C, UsersByEmail, 1> {
				UsersByEmailQuery {
					client,
					params: [email],
					stmt: &mut self.0,
					extractor: |row| UsersByEmailBorrowed {
						id: row.get(0),
						email: row.get(1),
						username: row.get(2),
						first_name: row.get(3),
						last_name: row.get(4),
						permission: row.get(5),
						system_admin: row.get(6),
						created_at: row.get(7),
						updated_at: row.get(8),
					},
					mapper: |it| <UsersByEmail>::from(it),
				}
			}
		}
		pub fn users_by_username() -> UsersByUsernameStmt {
			UsersByUsernameStmt(cornucopia_async::private::Stmt::new(
				"SELECT *
FROM users
WHERE username = $1
LIMIT 1",
			))
		}
		pub struct UsersByUsernameStmt(cornucopia_async::private::Stmt);
		impl UsersByUsernameStmt {
			pub fn bind<'a, C: GenericClient, T1: cornucopia_async::StringSql>(
				&'a mut self, client: &'a C, username: &'a T1,
			) -> UsersByUsernameQuery<'a, C, UsersByUsername, 1> {
				UsersByUsernameQuery {
					client,
					params: [username],
					stmt: &mut self.0,
					extractor: |row| UsersByUsernameBorrowed {
						id: row.get(0),
						email: row.get(1),
						username: row.get(2),
						first_name: row.get(3),
						last_name: row.get(4),
						permission: row.get(5),
						system_admin: row.get(6),
						created_at: row.get(7),
						updated_at: row.get(8),
					},
					mapper: |it| <UsersByUsername>::from(it),
				}
			}
		}
		pub fn update_user_permission() -> UpdateUserPermissionStmt {
			UpdateUserPermissionStmt(cornucopia_async::private::Stmt::new(
				"UPDATE users
SET 
    permission = $1::permission,
    updated_at = NOW()
WHERE id = $2
RETURNING *",
			))
		}
		pub struct UpdateUserPermissionStmt(cornucopia_async::private::Stmt);
		impl UpdateUserPermissionStmt {
			pub fn bind<'a, C: GenericClient>(
				&'a mut self, client: &'a C, permission: &'a super::super::types::public::Permission, id: &'a uuid::Uuid,
			) -> UpdateUserPermissionQuery<'a, C, UpdateUserPermission, 2> {
				UpdateUserPermissionQuery {
					client,
					params: [permission, id],
					stmt: &mut self.0,
					extractor: |row| UpdateUserPermissionBorrowed {
						id: row.get(0),
						email: row.get(1),
						username: row.get(2),
						first_name: row.get(3),
						last_name: row.get(4),
						permission: row.get(5),
						system_admin: row.get(6),
						created_at: row.get(7),
						updated_at: row.get(8),
					},
					mapper: |it| <UpdateUserPermission>::from(it),
				}
			}
		}
		impl<'a, C: GenericClient> cornucopia_async::Params<'a, UpdateUserPermissionParams, UpdateUserPermissionQuery<'a, C, UpdateUserPermission, 2>, C>
			for UpdateUserPermissionStmt
		{
			fn params(&'a mut self, client: &'a C, params: &'a UpdateUserPermissionParams) -> UpdateUserPermissionQuery<'a, C, UpdateUserPermission, 2> {
				self.bind(client, &params.permission, &params.id)
			}
		}
		pub fn update_user() -> UpdateUserStmt {
			UpdateUserStmt(cornucopia_async::private::Stmt::new(
				"UPDATE users
SET 
    email = COALESCE($1, email),
    username = COALESCE($2, username),
    first_name = COALESCE($3, first_name),
    last_name = COALESCE($4, last_name),
    updated_at = NOW()
WHERE id = $5
RETURNING *",
			))
		}
		pub struct UpdateUserStmt(cornucopia_async::private::Stmt);
		impl UpdateUserStmt {
			pub fn bind<
				'a,
				C: GenericClient,
				T1: cornucopia_async::StringSql,
				T2: cornucopia_async::StringSql,
				T3: cornucopia_async::StringSql,
				T4: cornucopia_async::StringSql,
			>(
				&'a mut self, client: &'a C, email: &'a T1, username: &'a T2, first_name: &'a T3, last_name: &'a T4, id: &'a uuid::Uuid,
			) -> UpdateUserQuery<'a, C, UpdateUser, 5> {
				UpdateUserQuery {
					client,
					params: [email, username, first_name, last_name, id],
					stmt: &mut self.0,
					extractor: |row| UpdateUserBorrowed {
						id: row.get(0),
						email: row.get(1),
						username: row.get(2),
						first_name: row.get(3),
						last_name: row.get(4),
						permission: row.get(5),
						system_admin: row.get(6),
						created_at: row.get(7),
						updated_at: row.get(8),
					},
					mapper: |it| <UpdateUser>::from(it),
				}
			}
		}
		impl<
				'a,
				C: GenericClient,
				T1: cornucopia_async::StringSql,
				T2: cornucopia_async::StringSql,
				T3: cornucopia_async::StringSql,
				T4: cornucopia_async::StringSql,
			> cornucopia_async::Params<'a, UpdateUserParams<T1, T2, T3, T4>, UpdateUserQuery<'a, C, UpdateUser, 5>, C> for UpdateUserStmt
		{
			fn params(&'a mut self, client: &'a C, params: &'a UpdateUserParams<T1, T2, T3, T4>) -> UpdateUserQuery<'a, C, UpdateUser, 5> {
				self.bind(client, &params.email, &params.username, &params.first_name, &params.last_name, &params.id)
			}
		}
		pub fn create_user() -> CreateUserStmt {
			CreateUserStmt(cornucopia_async::private::Stmt::new(
				"INSERT INTO users (
    email,
    username,
    first_name,
    last_name,
    permission
)
VALUES (
    $1,
    $2,
    $3,
    $4,
    $5::permission
)
RETURNING *",
			))
		}
		pub struct CreateUserStmt(cornucopia_async::private::Stmt);
		impl CreateUserStmt {
			pub fn bind<
				'a,
				C: GenericClient,
				T1: cornucopia_async::StringSql,
				T2: cornucopia_async::StringSql,
				T3: cornucopia_async::StringSql,
				T4: cornucopia_async::StringSql,
			>(
				&'a mut self, client: &'a C, email: &'a T1, username: &'a T2, first_name: &'a T3, last_name: &'a T4,
				permission: &'a super::super::types::public::Permission,
			) -> CreateUserQuery<'a, C, CreateUser, 5> {
				CreateUserQuery {
					client,
					params: [email, username, first_name, last_name, permission],
					stmt: &mut self.0,
					extractor: |row| CreateUserBorrowed {
						id: row.get(0),
						email: row.get(1),
						username: row.get(2),
						first_name: row.get(3),
						last_name: row.get(4),
						permission: row.get(5),
						system_admin: row.get(6),
						created_at: row.get(7),
						updated_at: row.get(8),
					},
					mapper: |it| <CreateUser>::from(it),
				}
			}
		}
		impl<
				'a,
				C: GenericClient,
				T1: cornucopia_async::StringSql,
				T2: cornucopia_async::StringSql,
				T3: cornucopia_async::StringSql,
				T4: cornucopia_async::StringSql,
			> cornucopia_async::Params<'a, CreateUserParams<T1, T2, T3, T4>, CreateUserQuery<'a, C, CreateUser, 5>, C> for CreateUserStmt
		{
			fn params(&'a mut self, client: &'a C, params: &'a CreateUserParams<T1, T2, T3, T4>) -> CreateUserQuery<'a, C, CreateUser, 5> {
				self.bind(client, &params.email, &params.username, &params.first_name, &params.last_name, &params.permission)
			}
		}
		pub fn bulk_insert_users() -> BulkInsertUsersStmt {
			BulkInsertUsersStmt(cornucopia_async::private::Stmt::new(
				"INSERT INTO users (
    email,
    username,
    first_name,
    last_name,
    permission
)
SELECT 
    (value->>'email')::text,
    (value->>'username')::text,  
    (value->>'first_name')::text,
    (value->>'last_name')::text,
    (value->>'permission')::permission
FROM json_array_elements($1::json)
RETURNING *",
			))
		}
		pub struct BulkInsertUsersStmt(cornucopia_async::private::Stmt);
		impl BulkInsertUsersStmt {
			pub fn bind<'a, C: GenericClient, T1: cornucopia_async::JsonSql>(
				&'a mut self, client: &'a C, users: &'a T1,
			) -> BulkInsertUsersQuery<'a, C, BulkInsertUsers, 1> {
				BulkInsertUsersQuery {
					client,
					params: [users],
					stmt: &mut self.0,
					extractor: |row| BulkInsertUsersBorrowed {
						id: row.get(0),
						email: row.get(1),
						username: row.get(2),
						first_name: row.get(3),
						last_name: row.get(4),
						permission: row.get(5),
						system_admin: row.get(6),
						created_at: row.get(7),
						updated_at: row.get(8),
					},
					mapper: |it| <BulkInsertUsers>::from(it),
				}
			}
		}
		pub fn delete_user() -> DeleteUserStmt {
			DeleteUserStmt(cornucopia_async::private::Stmt::new(
				"DELETE FROM users
WHERE id = $1
RETURNING *",
			))
		}
		pub struct DeleteUserStmt(cornucopia_async::private::Stmt);
		impl DeleteUserStmt {
			pub fn bind<'a, C: GenericClient>(&'a mut self, client: &'a C, id: &'a uuid::Uuid) -> DeleteUserQuery<'a, C, DeleteUser, 1> {
				DeleteUserQuery {
					client,
					params: [id],
					stmt: &mut self.0,
					extractor: |row| DeleteUserBorrowed {
						id: row.get(0),
						email: row.get(1),
						username: row.get(2),
						first_name: row.get(3),
						last_name: row.get(4),
						permission: row.get(5),
						system_admin: row.get(6),
						created_at: row.get(7),
						updated_at: row.get(8),
					},
					mapper: |it| <DeleteUser>::from(it),
				}
			}
		}
		pub fn search_users() -> SearchUsersStmt {
			SearchUsersStmt(cornucopia_async::private::Stmt::new(
				"SELECT *
FROM users
WHERE 
    (LOWER(username) LIKE LOWER($1) || '%' OR
    LOWER(email) LIKE LOWER($1) || '%' OR
    LOWER(first_name) LIKE LOWER($1) || '%' OR
    LOWER(last_name) LIKE LOWER($1) || '%')
ORDER BY $2 DESC
OFFSET $3
LIMIT $4",
			))
		}
		pub struct SearchUsersStmt(cornucopia_async::private::Stmt);
		impl SearchUsersStmt {
			pub fn bind<'a, C: GenericClient, T1: cornucopia_async::StringSql, T2: cornucopia_async::StringSql>(
				&'a mut self, client: &'a C, search: &'a T1, order_by: &'a T2, offset: &'a i64, limit: &'a i64,
			) -> SearchUsersQuery<'a, C, SearchUsers, 4> {
				SearchUsersQuery {
					client,
					params: [search, order_by, offset, limit],
					stmt: &mut self.0,
					extractor: |row| SearchUsersBorrowed {
						id: row.get(0),
						email: row.get(1),
						username: row.get(2),
						first_name: row.get(3),
						last_name: row.get(4),
						permission: row.get(5),
						system_admin: row.get(6),
						created_at: row.get(7),
						updated_at: row.get(8),
					},
					mapper: |it| <SearchUsers>::from(it),
				}
			}
		}
		impl<'a, C: GenericClient, T1: cornucopia_async::StringSql, T2: cornucopia_async::StringSql>
			cornucopia_async::Params<'a, SearchUsersParams<T1, T2>, SearchUsersQuery<'a, C, SearchUsers, 4>, C> for SearchUsersStmt
		{
			fn params(&'a mut self, client: &'a C, params: &'a SearchUsersParams<T1, T2>) -> SearchUsersQuery<'a, C, SearchUsers, 4> {
				self.bind(client, &params.search, &params.order_by, &params.offset, &params.limit)
			}
		}
	}
}
