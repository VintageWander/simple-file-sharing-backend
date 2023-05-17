use axum::{extract::State, routing::post, Router};

use crate::{
    error::Error,
    folder::request::create::CreateFolderRequest,
    prisma::{folder, user::Data, Visibility},
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
            let root_folder = db
                .folder()
                .find_first(vec![
                    folder::owner_id::equals(user_id.clone()),
                    folder::parent_folder_id::equals(None),
                ])
                .select(folder::select!({ id }))
                .exec()
                .await?
                .ok_or_else(|| Error::NotFound)?;

            let parent = match optional_parent {
                Some(parent) => parent.to_string(),
                None => root_folder.id,
            };

            let visibility = optional_visibility.unwrap_or(Visibility::Private);

            let new_folder = db
                .folder()
                .create_unchecked(
                    user_id,
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
