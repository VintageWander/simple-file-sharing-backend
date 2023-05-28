use axum::{extract::State, routing::get, Router};

use crate::{
    error::Error, extractors::param::ParamId, prisma::user, user::response::user_response,
    web::Web, GlobalState, WebResult,
};

pub fn get_users() -> Router<GlobalState> {
    async fn get_users_handler(State(GlobalState { db, .. }): State<GlobalState>) -> WebResult {
        let users = db
            .user()
            .find_many(vec![])
            .select(user_response::select())
            .exec()
            .await?;
        Ok(Web::ok("Get users successfully", users))
    }
    Router::new().route("/", get(get_users_handler))
}

pub fn get_user() -> Router<GlobalState> {
    async fn get_user_handler(
        State(GlobalState { db, .. }): State<GlobalState>,
        ParamId(user_id): ParamId,
    ) -> WebResult {
        let user = db
            .user()
            .find_unique(user::id::equals(user_id))
            .select(user_response::select())
            .exec()
            .await?
            .ok_or_else(|| Error::NotFound)?;
        Ok(Web::ok("Get user by id successfully", user))
    }
    Router::new().route("/:user_id", get(get_user_handler))
}
