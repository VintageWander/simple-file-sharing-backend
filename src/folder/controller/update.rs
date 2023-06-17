use axum::{extract::State, routing::put, Router};

use crate::{
    error::Error,
    extractors::param::ParamId,
    folder::model::update::UpdateFolderRequest,
    prisma::folder,
    user::model::{loggedin::LoggedInUser, select::UserSelect},
    web::Web,
    GlobalState, WebResult,
};

pub fn update_folder() -> Router<GlobalState> {
    async fn update_folder_handler(
        State(GlobalState {
            db, folder_service, ..
        }): State<GlobalState>,
        LoggedInUser(UserSelect { id: user_id, .. }): LoggedInUser,
        ParamId(param_folder_id): ParamId,
        UpdateFolderRequest {
            parent,
            folder_name,
            visibility,
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
        let target_folder = folder_service
            .get_folder_by_user_id(vec![folder::id::equals(param_folder_id)], user_id.clone())
            .await?;

        let parent_folder_id = match parent {
            Some(parent) => {
                let parent_folder = folder_service
                    .get_folder_by_user_id(vec![folder::id::equals(parent)], user_id)
                    .await?;

                if parent_folder.id == target_folder.id {
                    return Err(Error::Forbidden);
                }

                Some(parent_folder.id)
            }
            None => None,
        };

        let updated_folder = folder_service
            .update_folder(target_folder.id, parent_folder_id, folder_name, visibility)
            .await?;

        Ok(Web::ok("Update folder success", updated_folder))
    }
    Router::new().route("/update/:folder_id", put(update_folder_handler))
}
