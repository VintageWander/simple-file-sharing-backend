use axum::{extract::State, routing::get, Router};

use crate::{
    error::Error,
    prisma::user::{self, Data},
    user::{request::loggedin::LoggedInUser, response::user_response},
    web::Web,
    GlobalState, WebResult,
};

pub fn profile() -> Router<GlobalState> {
    async fn profile_handler(
        State(GlobalState { db, .. }): State<GlobalState>,
        LoggedInUser(Data { id: user_id, .. }): LoggedInUser,
    ) -> WebResult {
        let filtered_user = db
            .user()
            .find_unique(user::id::equals(user_id))
            .select(user_response::select())
            .exec()
            .await?
            .ok_or_else(|| Error::NotFound)?;
        Ok(Web::ok("Get user's profile success", filtered_user))
    }
    Router::new().route("/profile", get(profile_handler))
}
