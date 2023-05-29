use axum::Router;

use crate::GlobalState;

use self::get::public::get_public_folders;

pub mod create;
pub mod delete;
pub mod get;
pub mod update;

use {create::create_folder, delete::delete_folder, update::update_folder};

pub fn folder_routes() -> Router<GlobalState> {
    Router::new().nest(
        "/folder",
        Router::new()
            // /folder/public
            .merge(get_public_folders())
            // /folder/create
            .merge(create_folder())
            .merge(update_folder())
            .merge(delete_folder()),
    )
}
