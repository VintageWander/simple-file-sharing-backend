use axum::{extract::State, routing::put, Router};

use crate::{
    error::Error,
    extractors::param::ParamId,
    folder::request::update::UpdateFolderRequest,
    prisma::{folder, user::Data},
    user::request::loggedin::LoggedInUser,
    web::Web,
    Database, WebResult,
};

use super::FolderController;

impl FolderController {
    pub fn update_folder(&self) -> Router<Database> {
        async fn update_folder_handler(
            State(db): State<Database>,
            LoggedInUser(Data { id: user_id, .. }): LoggedInUser,
            ParamId(param_folder_id): ParamId,
            UpdateFolderRequest {
                parent,
                folder_name,
                visibility,
            }: UpdateFolderRequest,
        ) -> WebResult {
            let old_folder = db
                .folder()
                .find_unique(folder::id::equals(param_folder_id.clone()))
                .include(folder::include!({
                    owner : select {
                        id username email created_at updated_at
                    }
                }))
                .exec()
                .await?
                .ok_or_else(|| Error::NotFound)?;

            // Just return if the form is empty
            if let (None, None, None) = (&parent, &folder_name, &visibility) {
                return Ok(Web::ok("There is nothing to update", old_folder));
            }

            // Else - actually doing the update
            let mut actions = vec![];

            if let Some(parent) = parent {
                actions.push(folder::parent_folder_id::set(Some(parent.to_string())))
            }

            if let Some(folder_name) = folder_name {
                actions.push(folder::folder_name::set(folder_name))
            }

            if let Some(visibility) = visibility {
                actions.push(folder::visibility::set(visibility))
            }

            let updated_folder = db
                .folder()
                .update(folder::id::equals(param_folder_id), actions)
                .include(folder::include!({
                    owner : select {
                        id username email created_at updated_at
                    }
                }))
                .exec()
                .await?;
            Ok(Web::ok("Update folder success", updated_folder))
        }
        Router::new().route("/update/:folder_id", put(update_folder_handler))
    }
}
