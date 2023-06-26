use axum::Router;

use crate::GlobalState;

use self::{
    collab::set_folder_collaborators,
    get::{my::get_my_folders, public::get_public_folders, shared::get_shared_folders},
};

pub mod collab;
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
            // folder/my
            .merge(get_my_folders())
            // folder/shared
            .merge(get_shared_folders())
            // /folder/create
            .merge(create_folder())
            // /folder/update
            .merge(update_folder())
            // /folder/delete
            .merge(delete_folder())
            // /folder/collaborators
            .merge(set_folder_collaborators()),
    )
}
