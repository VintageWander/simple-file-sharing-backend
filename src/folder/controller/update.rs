use axum::{extract::State, routing::put, Router};

use crate::{
    error::Error,
    extractors::param::ParamId,
    folder::{request::update::UpdateFolderRequest, response::folder_response},
    prisma::{folder, user::Data},
    user::request::loggedin::LoggedInUser,
    web::Web,
    GlobalState, WebResult,
};

pub fn update_folder() -> Router<GlobalState> {
    async fn update_folder_handler(
        State(GlobalState { db, .. }): State<GlobalState>,
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
            .select(folder_response::select())
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
            .select(folder_response::select())
            .exec()
            .await?;

        Ok(Web::ok("Update folder success", ()))
    }
    Router::new().route("/update/:folder_id", put(update_folder_handler))
}
