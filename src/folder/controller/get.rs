use axum::{
    extract::{Query, State},
    routing::get,
    Router,
};
use serde::Deserialize;
use uuid::Uuid;

use crate::{folder::response::folder_response, prisma::folder, web::Web, GlobalState, WebResult};

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct FolderQuery {
    parent_folder_id: Option<Uuid>,
}

pub fn get_folders() -> Router<GlobalState> {
    // This function does not list all folders that exists in the database
    // But rather lists all folders in the root directory
    async fn get_folders_handler(
        State(GlobalState { db, .. }): State<GlobalState>,
        Query(FolderQuery { parent_folder_id }): Query<FolderQuery>,
    ) -> WebResult {
        // If the root folder provided in the param
        // Go find them
        if let Some(parent_folder_id) = parent_folder_id {
            let folders = db
                .folder()
                .find_many(vec![folder::parent_folder_id::equals(Some(
                    parent_folder_id.to_string(),
                ))])
                .select(folder_response::select())
                .exec()
                .await?;
            return Ok(Web::ok("Get all folders at position success", folders));
        }

        let folders = db
            .folder()
            .find_many(vec![folder::parent_folder_id::equals(None)])
            .select(folder_response::select())
            .exec()
            .await?
            .into_iter()
            .flat_map(|root_folder| root_folder.child_folders)
            .collect::<Vec<_>>();

        Ok(Web::ok("Get all folders successfully", folders))
    }
    Router::new().route("/", get(get_folders_handler))
}
