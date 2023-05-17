use axum::{
    extract::{Query, State},
    routing::get,
    Router,
};
use serde::Deserialize;
use uuid::Uuid;

use crate::{prisma::folder, web::Web, Database, WebResult};

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct FolderQuery {
    parent_folder_id: Option<Uuid>,
}

use super::FolderController;

impl FolderController {
    pub fn get_folders(&self) -> Router<Database> {
        // This function does not list all folders that exists in the database
        // But rather lists all folders in the root directory
        async fn get_folders_handler(
            State(db): State<Database>,
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
                    .select(folder::select!({
                        id
                        owner: select {
                            id
                            username
                            email
                            created_at
                            updated_at
                        }
                        parent_folder_id
                        folder_name
                        visibility
                        tags
                        created_at
                        updated_at
                    }))
                    .exec()
                    .await?;
                return Ok(Web::ok("Get all folders at position success", folders));
            }

            let folders = db
                .folder()
                .find_many(vec![folder::parent_folder_id::equals(None)])
                .select(folder::select!({
                    child_folders: select {
                        id
                        owner: select {
                            id
                            username
                            email
                            created_at
                            updated_at
                        }
                        parent_folder_id
                        folder_name
                        visibility
                        tags
                        created_at
                        updated_at
                    }
                }))
                .exec()
                .await?
                .into_iter()
                .flat_map(|root_folder| root_folder.child_folders)
                .collect::<Vec<_>>();

            Ok(Web::ok("Get all folders successfully", folders))
        }
        Router::new().route("/", get(get_folders_handler))
    }
}
