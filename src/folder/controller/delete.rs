use axum::{extract::State, Router};

use crate::{
    error::Error,
    extractors::param::ParamId,
    prisma::{folder, user::Data},
    user::request::loggedin::LoggedInUser,
    web::Web,
    Database, WebResult,
};

use super::FolderController;

impl FolderController {
    pub fn delete_folder(&self) -> Router<Database> {
        async fn delete_folder_handler(
            State(db): State<Database>,
            LoggedInUser(Data { id: user_id, .. }): LoggedInUser,
            ParamId(param_folder_id): ParamId,
        ) -> WebResult {
            let target = db
                .folder()
                .find_unique(folder::id::equals(param_folder_id))
                .select(folder::select!({
                    parent_folder_id owner_id folder_name
                }))
                .exec()
                .await?
                .ok_or_else(|| Error::NotFound)?;

            if (target.parent_folder_id.unwrap() == target.owner_id)
                && (target.owner_id == target.folder_name)
            {
                return Err(Error::Forbidden);
            }

            Ok(Web::ok("Deleted folder successfully", ()))
        }
        Router::new()
    }
}
