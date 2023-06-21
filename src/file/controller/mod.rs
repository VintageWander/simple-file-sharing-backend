pub mod create;
pub mod delete;
pub mod get;
pub mod restore;
pub mod update;

use axum::Router;

use crate::GlobalState;

use self::{
    create::create_file,
    delete::{delete_file, delete_file_version},
    get::{
        content::get_content, my::get_my_files, public::get_public_files, shared::get_shared_files,
    },
    restore::restore_file,
    update::update_file,
};

pub fn file_routes() -> Router<GlobalState> {
    Router::new().nest(
        "/file",
        Router::new()
            .merge(get_public_files())
            .merge(get_my_files())
            .merge(get_shared_files())
            .merge(get_content())
            .merge(create_file())
            .merge(update_file())
            .merge(delete_file())
            .merge(delete_file_version())
            .merge(restore_file()),
    )
}
