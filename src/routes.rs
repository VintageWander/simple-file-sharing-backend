use axum::{extract::State, routing::options, Router};

use crate::{
    auth::controller::auth_routes, file::controller::file_routes,
    folder::controller::folder_routes, tag::controller::tag_route, user::controller::user_routes,
    web::Web, GlobalState, WebResult,
};

fn preflight() -> Router<GlobalState> {
    async fn preflight_handler(_: State<GlobalState>) -> WebResult {
        Ok(Web::ok("Preflight request passed", ()))
    }
    Router::new().route("/", options(preflight_handler))
}

pub fn routes() -> Router<GlobalState> {
    Router::new()
        .merge(preflight())
        .merge(user_routes())
        .merge(auth_routes())
        .merge(folder_routes())
        .merge(file_routes())
        .merge(tag_route())
}
