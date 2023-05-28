pub mod get;

use axum::Router;

use crate::GlobalState;

use self::get::get_files;

pub fn file_routes() -> Router<GlobalState> {
    Router::new().nest("/file", Router::new().merge(get_files()))
}
