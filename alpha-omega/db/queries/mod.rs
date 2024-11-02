// This file was generated with `cornucopia`. Do not modify.

#[allow(clippy::all, clippy::pedantic)]
#[allow(unused_variables)]
#[allow(unused_imports)]
#[allow(dead_code)]
pub mod types {}
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
		pub struct SetNameParams<T1: cornucopia_async::StringSql, T2: cornucopia_async::StringSql> {
			pub first_name: T1,
			pub last_name: T2,
			pub current_user_id: uuid::Uuid,
		}
		#[derive(serde::Serialize, Debug, Clone, PartialEq)]
		pub struct User {
			pub id: uuid::Uuid,
			pub email: String,
			pub first_name: Option<String>,
			pub last_name: Option<String>,
			pub system_admin: bool,
		}
		pub struct UserBorrowed<'a> {
			pub id: uuid::Uuid,
			pub email: &'a str,
			pub first_name: Option<&'a str>,
			pub last_name: Option<&'a str>,
			pub system_admin: bool,
		}
		impl<'a> From<UserBorrowed<'a>> for User {
			fn from(UserBorrowed { id, email, first_name, last_name, system_admin }: UserBorrowed<'a>) -> Self {
				Self { id, email: email.into(), first_name: first_name.map(|v| v.into()), last_name: last_name.map(|v| v.into()), system_admin }
			}
		}
		pub struct UserQuery<'a, C: GenericClient, T, const N: usize> {
			client: &'a C,
			params: [&'a (dyn postgres_types::ToSql + Sync); N],
			stmt: &'a mut cornucopia_async::private::Stmt,
			extractor: fn(&tokio_postgres::Row) -> UserBorrowed,
			mapper: fn(UserBorrowed) -> T,
		}
		impl<'a, C, T: 'a, const N: usize> UserQuery<'a, C, T, N>
		where
			C: GenericClient,
		{
			pub fn map<R>(self, mapper: fn(UserBorrowed) -> R) -> UserQuery<'a, C, R, N> {
				UserQuery { client: self.client, params: self.params, stmt: self.stmt, extractor: self.extractor, mapper }
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
		pub struct UuidUuidQuery<'a, C: GenericClient, T, const N: usize> {
			client: &'a C,
			params: [&'a (dyn postgres_types::ToSql + Sync); N],
			stmt: &'a mut cornucopia_async::private::Stmt,
			extractor: fn(&tokio_postgres::Row) -> uuid::Uuid,
			mapper: fn(uuid::Uuid) -> T,
		}
		impl<'a, C, T: 'a, const N: usize> UuidUuidQuery<'a, C, T, N>
		where
			C: GenericClient,
		{
			pub fn map<R>(self, mapper: fn(uuid::Uuid) -> R) -> UuidUuidQuery<'a, C, R, N> {
				UuidUuidQuery { client: self.client, params: self.params, stmt: self.stmt, extractor: self.extractor, mapper }
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
		pub struct GetByEmail {
			pub id: uuid::Uuid,
			pub email: String,
			pub first_name: Option<String>,
			pub last_name: Option<String>,
		}
		pub struct GetByEmailBorrowed<'a> {
			pub id: uuid::Uuid,
			pub email: &'a str,
			pub first_name: Option<&'a str>,
			pub last_name: Option<&'a str>,
		}
		impl<'a> From<GetByEmailBorrowed<'a>> for GetByEmail {
			fn from(GetByEmailBorrowed { id, email, first_name, last_name }: GetByEmailBorrowed<'a>) -> Self {
				Self { id, email: email.into(), first_name: first_name.map(|v| v.into()), last_name: last_name.map(|v| v.into()) }
			}
		}
		pub struct GetByEmailQuery<'a, C: GenericClient, T, const N: usize> {
			client: &'a C,
			params: [&'a (dyn postgres_types::ToSql + Sync); N],
			stmt: &'a mut cornucopia_async::private::Stmt,
			extractor: fn(&tokio_postgres::Row) -> GetByEmailBorrowed,
			mapper: fn(GetByEmailBorrowed) -> T,
		}
		impl<'a, C, T: 'a, const N: usize> GetByEmailQuery<'a, C, T, N>
		where
			C: GenericClient,
		{
			pub fn map<R>(self, mapper: fn(GetByEmailBorrowed) -> R) -> GetByEmailQuery<'a, C, R, N> {
				GetByEmailQuery { client: self.client, params: self.params, stmt: self.stmt, extractor: self.extractor, mapper }
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
		pub fn user() -> UserStmt {
			UserStmt(cornucopia_async::private::Stmt::new(
				"SELECT 
    id, email, first_name, last_name, system_admin
FROM 
    users
WHERE
    id = $1",
			))
		}
		pub struct UserStmt(cornucopia_async::private::Stmt);
		impl UserStmt {
			pub fn bind<'a, C: GenericClient>(&'a mut self, client: &'a C, id: &'a uuid::Uuid) -> UserQuery<'a, C, User, 1> {
				UserQuery {
					client,
					params: [id],
					stmt: &mut self.0,
					extractor: |row| UserBorrowed { id: row.get(0), email: row.get(1), first_name: row.get(2), last_name: row.get(3), system_admin: row.get(4) },
					mapper: |it| <User>::from(it),
				}
			}
		}
		pub fn insert() -> InsertStmt {
			InsertStmt(cornucopia_async::private::Stmt::new(
				"INSERT INTO 
    users (email)
VALUES($1) 
RETURNING id",
			))
		}
		pub struct InsertStmt(cornucopia_async::private::Stmt);
		impl InsertStmt {
			pub fn bind<'a, C: GenericClient, T1: cornucopia_async::StringSql>(&'a mut self, client: &'a C, email: &'a T1) -> UuidUuidQuery<'a, C, uuid::Uuid, 1> {
				UuidUuidQuery { client, params: [email], stmt: &mut self.0, extractor: |row| row.get(0), mapper: |it| it }
			}
		}
		pub fn get_by_email() -> GetByEmailStmt {
			GetByEmailStmt(cornucopia_async::private::Stmt::new(
				"SELECT 
    id, email, first_name, last_name
FROM 
    users
WHERE
    email = $1",
			))
		}
		pub struct GetByEmailStmt(cornucopia_async::private::Stmt);
		impl GetByEmailStmt {
			pub fn bind<'a, C: GenericClient, T1: cornucopia_async::StringSql>(&'a mut self, client: &'a C, email: &'a T1) -> GetByEmailQuery<'a, C, GetByEmail, 1> {
				GetByEmailQuery {
					client,
					params: [email],
					stmt: &mut self.0,
					extractor: |row| GetByEmailBorrowed { id: row.get(0), email: row.get(1), first_name: row.get(2), last_name: row.get(3) },
					mapper: |it| <GetByEmail>::from(it),
				}
			}
		}
		pub fn set_name() -> SetNameStmt {
			SetNameStmt(cornucopia_async::private::Stmt::new(
				"UPDATE
    users
SET 
    first_name = $1, last_name = $2
WHERE
    id = $3",
			))
		}
		pub struct SetNameStmt(cornucopia_async::private::Stmt);
		impl SetNameStmt {
			pub async fn bind<'a, C: GenericClient, T1: cornucopia_async::StringSql, T2: cornucopia_async::StringSql>(
				&'a mut self, client: &'a C, first_name: &'a T1, last_name: &'a T2, current_user_id: &'a uuid::Uuid,
			) -> Result<u64, tokio_postgres::Error> {
				let stmt = self.0.prepare(client).await?;
				client.execute(stmt, &[first_name, last_name, current_user_id]).await
			}
		}
		impl<'a, C: GenericClient + Send + Sync, T1: cornucopia_async::StringSql, T2: cornucopia_async::StringSql>
			cornucopia_async::Params<'a, SetNameParams<T1, T2>, std::pin::Pin<Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>>, C>
			for SetNameStmt
		{
			fn params(
				&'a mut self, client: &'a C, params: &'a SetNameParams<T1, T2>,
			) -> std::pin::Pin<Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>> {
				Box::pin(self.bind(client, &params.first_name, &params.last_name, &params.current_user_id))
			}
		}
		pub fn count_users() -> CountUsersStmt {
			CountUsersStmt(cornucopia_async::private::Stmt::new(
				"SELECT
    count(id)
FROM
    users",
			))
		}
		pub struct CountUsersStmt(cornucopia_async::private::Stmt);
		impl CountUsersStmt {
			pub fn bind<'a, C: GenericClient>(&'a mut self, client: &'a C) -> I64Query<'a, C, i64, 0> {
				I64Query { client, params: [], stmt: &mut self.0, extractor: |row| row.get(0), mapper: |it| it }
			}
		}
	}
}
