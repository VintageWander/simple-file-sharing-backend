pub mod create;
pub mod get;

use axum::Router;

use crate::GlobalState;

use self::get::{my::get_my_files, public::get_public_files, shared::get_shared_files};

pub fn file_routes() -> Router<GlobalState> {
    Router::new().nest(
        "/file",
        Router::new()
            .merge(get_public_files())
            .merge(get_my_files())
            .merge(get_shared_files()),
    )
}
