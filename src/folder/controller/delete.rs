use axum::{extract::State, routing::delete, Router};

use crate::{
    error::Error,
    extractors::param::ParamId,
    prisma::{folder, user::Data},
    user::request::loggedin::LoggedInUser,
    web::Web,
    GlobalState, WebResult,
};

pub fn delete_folder() -> Router<GlobalState> {
    async fn delete_folder_handler(
        State(GlobalState { db, .. }): State<GlobalState>,
        LoggedInUser(Data { id: user_id, .. }): LoggedInUser,
        ParamId(param_folder_id): ParamId,
    ) -> WebResult {
        let target = db
            .folder()
            .find_unique(folder::id::equals(param_folder_id))
            .select(folder::select!({ id parent_folder_id }))
            .exec()
            .await?
            .ok_or_else(|| Error::NotFound)?;

        // Match the status of the target folder
        match target.parent_folder_id.is_none() {
            // If it is None -> Root Folder
            true => Err(Error::Forbidden),
            // Else -> Delete
            false => {
                db.folder()
                    .delete(folder::id::equals(target.id))
                    .exec()
                    .await?;

                Ok(Web::ok("Deleted folder successfully", ()))
            }
        }
    }
    Router::new().route("/delete/:folder_id", delete(delete_folder_handler))
}
