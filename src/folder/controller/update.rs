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
                .find_unique(folder::id::equals(param_folder_id))
                .select(folder::select!({
                    id
                    parent_folder_id
                    owner : select {
                        id username email created_at updated_at
                    }
                }))
                .exec()
                .await?
                .ok_or_else(|| Error::NotFound)?;

            if old_folder.parent_folder_id.is_none() {
                return Err(Error::Forbidden);
            }

            if let (&None, &None, &None) = (&parent, &folder_name, &visibility) {
                return Ok(Web::ok("Theres nothing to be updated", old_folder));
            }

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
                .update_unchecked(folder::id::equals(old_folder.id), actions)
                .include(folder::include!({ owner: select {
                    id username email created_at updated_at
                }}))
                .exec()
                .await?;

            Ok(Web::ok("Update folder success", ()))
        }
        Router::new().route("/update/:folder_id", put(update_folder_handler))
    }
}
