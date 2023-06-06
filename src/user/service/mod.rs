use std::sync::Arc;

use crate::{
    error::Error,
    prisma::{
        user::{self, SetParam, WhereParam},
        PrismaClient,
    },
};

use super::response::user_response;

#[derive(Clone)]
pub struct UserService {
    db: Arc<PrismaClient>,
}

impl UserService {
    pub fn init(db: &Arc<PrismaClient>) -> Self {
        Self { db: db.clone() }
    }

    pub async fn get_users(
        &self,
        filters: Vec<WhereParam>,
    ) -> Result<Vec<user_response::Data>, Error> {
        let users = self
            .db
            .user()
            .find_many(filters)
            .select(user_response::select())
            .exec()
            .await?;
        Ok(users)
    }

    pub async fn get_user_by_login_info(
        &self,
        username: String,
        password: String,
    ) -> Result<user_response::Data, Error> {
        self.db
            .user()
            .find_first(vec![
                user::username::equals(username),
                user::password::equals(password),
            ])
            .select(user_response::select())
            .exec()
            .await?
            .ok_or_else(|| Error::NotFound)
    }

    pub async fn get_user_by_id(&self, user_id: String) -> Result<user_response::Data, Error> {
        let user = self
            .db
            .user()
            .find_unique(user::id::equals(user_id))
            .select(user_response::select())
            .exec()
            .await?
            .ok_or_else(|| Error::NotFound)?;
        Ok(user)
    }

    pub async fn get_user_by_id_with_password(&self, user_id: String) -> Result<user::Data, Error> {
        let user = self
            .db
            .user()
            .find_unique(user::id::equals(user_id))
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
    ) -> Result<user_response::Data, Error> {
        self.db
            .user()
            .create(username, email, password, vec![])
            .select(user_response::select())
            .exec()
            .await
            .map_err(Into::into)
    }

    pub async fn update_user(
        &self,
        user_id: String,
        changes: Vec<SetParam>,
    ) -> Result<user_response::Data, Error> {
        self.db
            .user()
            .update(user::id::equals(user_id), changes)
            .select(user_response::select())
            .exec()
            .await
            .map_err(Into::into)
    }

    pub async fn delete_user(&self, user_id: String) -> Result<(), Error> {
        self.db
            .user()
            .delete(user::id::equals(user_id))
            .select(user_response::select())
            .exec()
            .await?;
        Ok(())
    }
}
