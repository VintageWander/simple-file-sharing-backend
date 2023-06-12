use axum::{extract::State, routing::get, Router};

use crate::{
    folder::model::query::FolderQuery,
    prisma::{folder, Visibility},
    web::Web,
    GlobalState, WebResult,
};

/*
    This route will discard visibility search in the query string
    And sets visibility to Public

*/

/*
    On the handlers side
    We only have to deal with owner_id, parent, and visiblity
*/

pub fn get_public_folders() -> Router<GlobalState> {
    /*
        This function does not list all folders that exists in the database
        But rather lists all folders in the root directory
    */
    async fn get_public_folders_handler(
        State(GlobalState { folder_service, .. }): State<GlobalState>,
        FolderQuery {
            owner_id,
            parent_folder_id,
            mut filters,
            ..
        }: FolderQuery,
    ) -> WebResult {
        filters.push(folder::visibility::equals(Visibility::Public));

        if let Some(owner_id) = owner_id {
            filters.push(folder::owner_id::equals(owner_id))
        }

        /*
            If the parent folder id is provided, then that should be a starting point folder
            to search forward
            Else just picks all root folders from the system
        */

        let folders = folder_service
            .get_child_folders_from_folders(parent_folder_id, filters)
            .await?;

        Ok(Web::ok("Get all public folders successfully", folders))
    }
    Router::new().route("/public", get(get_public_folders_handler))
}
