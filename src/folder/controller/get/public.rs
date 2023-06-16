use axum::{extract::State, routing::get, Router};

use crate::{
    folder::model::query::FolderQuery, prisma::Visibility, web::Web, GlobalState, WebResult,
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
            id,
            owner_id,
            parent_folder_id,
            folder_name,
            created_at,
            updated_at,
            ..
        }: FolderQuery,
    ) -> WebResult {
        /*
            If the parent folder id is provided, then that should be a starting point folder
            to search forward
            Else just picks all root folders from the system
        */

        let folders = folder_service
            .get_child_folders_from_folders(
                id,
                owner_id,
                parent_folder_id,
                folder_name,
                Some(Visibility::Public),
                created_at,
                updated_at,
            )
            .await?;

        Ok(Web::ok("Get all public folders successfully", folders))
    }
    Router::new().route("/public", get(get_public_folders_handler))
}
