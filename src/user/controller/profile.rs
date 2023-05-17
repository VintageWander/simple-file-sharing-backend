use axum::{extract::State, routing::get, Router};

use crate::{
    error::Error,
    prisma::user::{self, Data},
    user::request::loggedin::LoggedInUser,
    web::Web,
    Database, WebResult,
};

use super::UserController;

impl UserController {
    pub fn profile(&self) -> Router<Database> {
        async fn profile_handler(
            State(db): State<Database>,
            LoggedInUser(Data { id: user_id, .. }): LoggedInUser,
        ) -> WebResult {
            let filtered_user = db
                .user()
                .find_unique(user::id::equals(user_id))
                .select(user::select!({
                    id username email created_at updated_at
                }))
                .exec()
                .await?
                .ok_or_else(|| Error::NotFound)?;
            Ok(Web::ok("Get user's profile success", filtered_user))
        }
        Router::new().route("/profile", get(profile_handler))
    }
}
