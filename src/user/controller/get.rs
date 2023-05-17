use axum::{extract::State, routing::get, Router};

use crate::{
    error::Error, extractors::param::ParamId, prisma::user, web::Web, Database, WebResult,
};

use super::UserController;

impl UserController {
    pub fn get_users(&self) -> Router<Database> {
        async fn get_users_handler(State(db): State<Database>) -> WebResult {
            let users = db
                .user()
                .find_many(vec![])
                .select(user::select!({
                    id username email created_at updated_at
                }))
                .exec()
                .await?;
            Ok(Web::ok("Get users successfully", users))
        }
        Router::new().route("/", get(get_users_handler))
    }

    pub fn get_user(&self) -> Router<Database> {
        async fn get_user_handler(
            State(db): State<Database>,
            ParamId(user_id): ParamId,
        ) -> WebResult {
            let user = db
                .user()
                .find_unique(user::id::equals(user_id))
                .select(user::select!({
                    id username email created_at updated_at
                }))
                .exec()
                .await?
                .ok_or_else(|| Error::NotFound)?;
            Ok(Web::ok("Get user by id successfully", user))
        }
        Router::new().route("/:user_id", get(get_user_handler))
    }
}
