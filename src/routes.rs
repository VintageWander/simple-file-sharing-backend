use axum::Router;

use crate::{
    auth::controller::auth_routes, file::controller::file_routes,
    folder::controller::folder_routes, user::controller::user_routes, GlobalState,
};

pub fn routes() -> Router<GlobalState> {
    Router::new()
        .merge(user_routes())
        .merge(auth_routes())
        .merge(folder_routes())
        .merge(file_routes())
}
