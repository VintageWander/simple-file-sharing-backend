pub mod create;
pub mod delete;
pub mod get;
pub mod update;

use axum::Router;

use crate::GlobalState;

use self::{
    create::create_file,
    delete::delete_file,
    get::{my::get_my_files, public::get_public_files, shared::get_shared_files},
    update::update_file,
};

pub fn file_routes() -> Router<GlobalState> {
    Router::new().nest(
        "/file",
        Router::new()
            .merge(get_public_files())
            .merge(get_my_files())
            .merge(get_shared_files())
            .merge(create_file())
            .merge(update_file())
            .merge(delete_file()),
    )
}
