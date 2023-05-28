use axum::{
    extract::{Query, State},
    routing::get,
    Router,
};
use serde::Deserialize;
use uuid::Uuid;

use crate::{
    prisma::{file, folder, user},
    web::Web,
    GlobalState, WebResult,
};

#[derive(Deserialize)]
struct FileQuery {
    pub parent_folder_id: Option<Uuid>,
}

pub fn get_files() -> Router<GlobalState> {
    async fn get_files_handler(
        State(GlobalState { db, .. }): State<GlobalState>,
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

        let files = db
            .folder()
            .find_many(vec![folder::parent_folder_id::equals(None)])
            .select(folder::select!({
                child_files: select {
                    id
                    owner: select {
                        id
                        username
                        email
                        created_at
                        updated_at
                    }
                    parent_folder_id
                    filename
                    extension
                    visibility
                    tags
                    versions
                    created_at
                    updated_at
                }
            }))
            .exec()
            .await?;

        Ok(Web::ok("Get all files successfully", files))
    }
    Router::new().route("/", get(get_files_handler))
}
