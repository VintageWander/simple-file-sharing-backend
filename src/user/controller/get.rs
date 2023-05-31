use axum::{extract::State, routing::get, Router};

use crate::{extractors::param::ParamId, web::Web, GlobalState, WebResult};

pub fn get_users() -> Router<GlobalState> {
    async fn get_users_handler(
        State(GlobalState { user_service, .. }): State<GlobalState>,
    ) -> WebResult {
        let users = user_service.get_users(vec![]).await?;
        Ok(Web::ok("Get users successfully", users))
    }
    Router::new().route("/", get(get_users_handler))
}

pub fn get_user() -> Router<GlobalState> {
    async fn get_user_handler(
        State(GlobalState { user_service, .. }): State<GlobalState>,
        ParamId(user_id): ParamId,
    ) -> WebResult {
        let user = user_service.get_user_by_id(user_id).await?;
        Ok(Web::ok("Get user by id successfully", user))
    }
    Router::new().route("/:user_id", get(get_user_handler))
}
