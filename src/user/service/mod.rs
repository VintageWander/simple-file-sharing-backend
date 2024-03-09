use std::sync::Arc;

use argon2::{
	password_hash::{rand_core::OsRng, SaltString},
	Argon2, PasswordHasher,
};

use crate::{
	error::Error,
	prisma::{
		user::{self, SetParam, WhereParam},
		PrismaClient,
	},
};

use super::model::select::{
	user_select, user_select_with_password, UserSelect, UserSelectWithPassword,
};

#[derive(Clone)]
pub struct UserService {
	pub db: Arc<PrismaClient>,
	salt: SaltString,
}

impl UserService {
	pub fn init(db: &Arc<PrismaClient>) -> Self {
		Self {
			db: db.clone(),
			salt: SaltString::generate(&mut OsRng),
		}
	}

	pub async fn get_users(&self, filters: Vec<WhereParam>) -> Result<Vec<UserSelect>, Error> {
		let users = self
			.db
			.user()
			.find_many(filters)
			.select(user_select::select())
			.exec()
			.await?;
		Ok(users)
	}

	pub async fn get_user_by_login_info(
		&self,
		username: String,
		password: String,
	) -> Result<UserSelect, Error> {
		let password = Argon2::default()
			.hash_password(password.as_bytes(), &self.salt)?
			.to_string();
		self.db
			.user()
			.find_first(vec![
				user::username::equals(username),
				user::password::equals(password),
			])
			.select(user_select::select())
			.exec()
			.await?
			.ok_or_else(|| Error::NotFound)
	}

	pub async fn get_user_by_id(&self, user_id: String) -> Result<UserSelect, Error> {
		let user = self
			.db
			.user()
			.find_unique(user::id::equals(user_id))
			.select(user_select::select())
			.exec()
			.await?
			.ok_or_else(|| Error::NotFound)?;

		Ok(user)
	}

	pub async fn get_user_by_id_with_password(
		&self,
		user_id: String,
	) -> Result<UserSelectWithPassword, Error> {
		let user = self
			.db
			.user()
			.find_unique(user::id::equals(user_id))
			.select(user_select_with_password::select())
			.exec()
			.await?
			.ok_or_else(|| Error::NotFound)?;

		Ok(user)
	}

	pub async fn create_user(
		&self,
		username: String,
		email: String,
		password: String,
	) -> Result<UserSelect, Error> {
		let password = Argon2::default()
			.hash_password(password.as_bytes(), &self.salt)?
			.to_string();
		self.db
			.user()
			.create(username, email, password, vec![])
			.select(user_select::select())
			.exec()
			.await
			.map_err(Into::into)
	}

	pub async fn update_user(
		&self,
		user_id: String,
		changes: Vec<SetParam>,
	) -> Result<UserSelect, Error> {
		self.db
			.user()
			.update(user::id::equals(user_id), changes)
			.select(user_select::select())
			.exec()
			.await
			.map_err(Into::into)
	}

	pub async fn delete_user(&self, user_id: String) -> Result<UserSelect, Error> {
		let deleted_user = self
			.db
			.user()
			.delete(user::id::equals(user_id))
			.select(user_select::select())
			.exec()
			.await?;

		Ok(deleted_user)
	}
}
