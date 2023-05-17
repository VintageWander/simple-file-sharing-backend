use axum::{extract::State, routing::post, Router};

use crate::{
    folder::request::create::CreateFolderRequest,
    prisma::{
        folder,
        user::{self, Data},
        Visibility,
    },
    user::request::loggedin::LoggedInUser,
    web::Web,
    Database, WebResult,
};

use super::FolderController;

impl FolderController {
    pub fn create_folder(&self) -> Router<Database> {
        async fn create_folder_handler(
            State(db): State<Database>,
            LoggedInUser(Data { id: user_id, .. }): LoggedInUser,
            CreateFolderRequest {
                parent: optional_parent,
                folder_name,
                visibility: optional_visibility,
            }: CreateFolderRequest,
        ) -> WebResult {
            let parent = match optional_parent {
                Some(parent) => parent.to_string(),
                None => user_id.clone(),
            };
            let visibility = optional_visibility.unwrap_or(Visibility::Public);

            let new_folder = db
                .folder()
                .create(
                    user::id::equals(user_id),
                    folder_name,
                    visibility,
                    vec![folder::parent_folder_id::set(Some(parent))],
                )
                .include(folder::include!({
                    owner: select {
                        id username email created_at updated_at
                    }
                }))
                .exec()
                .await?;

            Ok(Web::created("Create new folder successfully", new_folder))
        }
        Router::new().route("/create", post(create_folder_handler))
    }
}
