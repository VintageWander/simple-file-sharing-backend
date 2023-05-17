use axum::{
    extract::{Query, State},
    routing::get,
    Router,
};
use serde::Deserialize;
use uuid::Uuid;

use crate::{
    error::Error,
    prisma::{file, folder, user},
    web::Web,
    Database, WebResult,
};

#[derive(Deserialize)]
struct FileQuery {
    pub parent_folder_id: Option<Uuid>,
}

use super::FileController;

impl FileController {
    pub fn get_files(&self) -> Router<Database> {
        async fn get_files_handler(
            State(db): State<Database>,
            Query(FileQuery { parent_folder_id }): Query<FileQuery>,
        ) -> WebResult {
            if let Some(parent_folder_id) = parent_folder_id {
                let files = db
                    .file()
                    .find_many(vec![file::parent_folder_id::equals(
                        parent_folder_id.to_string(),
                    )])
                    .select(file::select!({
                        owner : select {
                            id username email created_at updated_at
                        }
                    }))
                    .exec()
                    .await?;
                return Ok(Web::ok("Get all folders at position success", files));
            }

            let users = db
                .user()
                .find_many(vec![])
                .select(user::select!({ id }))
                .exec()
                .await?;

            let mut root_folders = vec![];
            for user in users {
                root_folders.push(
                    db.folder()
                        .find_first(vec![folder::parent_folder_id::equals(None)])
                        .select(folder::select!({ id }))
                        .exec()
                        .await?
                        .ok_or_else(|| Error::NotFound)?,
                )
            }

            let mut results = vec![];
            for root_folder in root_folders {
                results.push(
                    db.file()
                        .find_many(vec![file::parent_folder_id::equals(root_folder.id)])
                        .include(file::include!({
                            owner : select {
                                id username email created_at updated_at
                            }
                        }))
                        .exec()
                        .await?,
                )
            }

            Ok(Web::ok("Get all files successfully", results))
        }
        Router::new().route("/", get(get_files_handler))
    }
}
