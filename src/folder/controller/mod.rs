use axum::Router;

use crate::GlobalState;

use self::{
    collab::set_folder_collaborators,
    content::get_folder_content,
    get::{my::get_my_folders, public::get_public_folders, shared::get_shared_folders},
};

pub mod collab;
pub mod content;
pub mod create;
pub mod delete;
pub mod get;
pub mod update;

use {create::create_folder, delete::delete_folder, update::update_folder};

pub fn folder_routes() -> Router<GlobalState> {
    Router::new().nest(
        "/folder",
        Router::new()
            // GET /folder/public?query
            .merge(get_public_folders())
            // GET folder/my?query
            .merge(get_my_folders())
            // GET folder/shared?query
            .merge(get_shared_folders())
            // POST /folder/create
            .merge(create_folder())
            // PUT /folder/update/:folder_id
            .merge(update_folder())
            // DELETE /folder/delete
            .merge(delete_folder())
            // PUT /folder/collaborators
            .merge(set_folder_collaborators())
            // GET /folder/content/:folder_id
            .merge(get_folder_content()),
    )
}
