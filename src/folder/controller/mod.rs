use axum::Router;

use crate::GlobalState;

pub mod create;
pub mod delete;
pub mod get;
pub mod update;

use {create::create_folder, delete::delete_folder, get::get_folders, update::update_folder};

pub fn folder_routes() -> Router<GlobalState> {
    Router::new().nest(
        "/folder",
        Router::new()
            // /folder?parentFolderId={folder id}
            .merge(get_folders())
            // /folder/create
            .merge(create_folder())
            .merge(update_folder())
            .merge(delete_folder()),
    )
}
