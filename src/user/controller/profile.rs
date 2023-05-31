use axum::{extract::State, routing::get, Router};

use crate::{
    user::{request::loggedin::LoggedInUser, response::user_response::Data},
    web::Web,
    GlobalState, WebResult,
};

pub fn profile() -> Router<GlobalState> {
    async fn profile_handler(
        State(GlobalState {
            db, user_service, ..
        }): State<GlobalState>,
        LoggedInUser(Data { id: user_id, .. }): LoggedInUser,
    ) -> WebResult {
        let filtered_user = user_service.get_user_by_id(user_id).await?;
        Ok(Web::ok("Get user's profile success", filtered_user))
    }
    Router::new().route("/profile", get(profile_handler))
}
