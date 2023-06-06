use axum::{extract::State, routing::put, Router};

use crate::{
    error::Error,
    extractors::param::ParamId,
    folder::{request::update::UpdateFolderRequest, response::folder_response},
    prisma::folder,
    user::{request::loggedin::LoggedInUser, response::user_response::Data},
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
            mut actions,
            ..
        }: UpdateFolderRequest,
    ) -> WebResult {
        /*
            Remember to use update_unchecked

            There are two important tasks when it comes to updating a folder
            If the parent field is provided,
            we have to make sure that the parent id
            points to a folder that is ours, OR shared to us

        */

        // Find the folder
        let found_folder = db
            .folder()
            .find_first(vec![folder::id::equals(param_folder_id)])
            .select(folder::select!({ id owner_id collaborators }))
            .exec()
            .await?
            .ok_or_else(|| Error::NotFound)?;

        // Ensure that it is ours, OR shared to us
        let is_ours = if found_folder.owner_id == user_id {
            true
        } else {
            found_folder
                .collaborators
                .into_iter()
                .any(|user| user.id == user_id)
        };

        // We cannot proceed with this action
        if !is_ours {
            return Err(Error::Forbidden);
        }

        // Here we can assume that the found_folder is ours to update

        // Let's move on to processing the parent field in the request
        let Some(parent) = parent else {
            /*
                If the parent field isn't present 
                Then we don't have to do anything much just update
            */
            let updated_folder = db
                .folder()
                .update_unchecked(folder::id::equals(found_folder.id), actions)
                .select(folder_response::select())
                .exec()
                .await?;

            return Ok(Web::ok("Update folder success", updated_folder));
        };

        let parent_folder = db
            .folder()
            .find_unique(folder::id::equals(parent))
            .select(folder::select!({ id owner_id collaborators }))
            .exec()
            .await?
            .ok_or_else(|| Error::NotFound)?;

        let is_ours = if parent_folder.owner_id == user_id {
            true
        } else {
            parent_folder
                .collaborators
                .into_iter()
                .any(|user| user.id == user_id)
        };

        if !is_ours {
            return Err(Error::NotFound);
        }

        actions.push(folder::parent_folder_id::set(Some(parent_folder.id)));

        let updated_folder = db
            .folder()
            .update_unchecked(folder::id::equals(found_folder.id), actions)
            .select(folder_response::select())
            .exec()
            .await?;

        Ok(Web::ok("Update folder success", updated_folder))
    }
    Router::new().route("/update/:folder_id", put(update_folder_handler))
}
