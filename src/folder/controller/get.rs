use axum::{
    extract::{Query, State},
    routing::get,
    Router,
};

use crate::{
    error::Error,
    prisma::{folder, user},
    web::Web,
    Database, WebResult,
};

use super::FolderController;

impl FolderController {
    pub fn get_folders(&self) -> Router<Database> {
        // This function does not list all folders that exists in the database
        // But rather lists all folders in the root directory
        async fn get_folders_handler(
            State(db): State<Database>,
            Query(param_root_folder_id_optional): Query<Option<String>>,
        ) -> WebResult {
            // If the root folder provided in the param
            // Go find them
            if let Some(param_root_folder_id) = param_root_folder_id_optional {
                let folders = db
                    .folder()
                    .find_many(vec![folder::parent_folder_id::equals(Some(
                        param_root_folder_id,
                    ))])
                    .include(folder::include!({
                        owner: select {
                            id username email created_at updated_at
                        }
                    }))
                    .exec()
                    .await?;
                return Ok(Web::ok("Get all folders at path success", folders));
            }

            // Else
            // Get all folders NEXT to the root folder
            // By
            // Get all users
            let users = db
                .user()
                .find_many(vec![])
                .select(user::select!({ id }))
                .exec()
                .await?;

            let mut root_folders = vec![];

            // Get all root folders
            for user in users {
                root_folders.push(
                    db.folder()
                        .find_first(vec![folder::parent_folder_id::equals(Some(user.id))])
                        .select(folder::select!({ id }))
                        .exec()
                        .await?
                        .ok_or_else(|| Error::NotFound)?,
                )
            }

            let mut results = vec![];
            // Get the folder that has a root folder as parent
            for root_folder in root_folders {
                results.push(
                    db.folder()
                        .find_first(vec![folder::parent_folder_id::equals(Some(root_folder.id))])
                        .include(folder::include!({
                            owner: select {
                                id username email created_at updated_at
                            }
                        }))
                        .exec()
                        .await?
                        .ok_or_else(|| Error::NotFound)?,
                )
            }

            Ok(Web::ok("Get all folders successfully", results))
        }
        Router::new().route("/", get(get_folders_handler))
    }
}
