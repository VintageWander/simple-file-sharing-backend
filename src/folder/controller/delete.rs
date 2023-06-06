use axum::{extract::State, routing::delete, Router};

use crate::{
    error::Error,
    extractors::param::ParamId,
    prisma::folder,
    user::{request::loggedin::LoggedInUser, response::user_response::Data},
    web::Web,
    GlobalState, WebResult,
};

pub fn delete_folder() -> Router<GlobalState> {
    async fn delete_folder_handler(
        State(GlobalState {
            db,
            folder_service,
            storage,
            ..
        }): State<GlobalState>,
        LoggedInUser(Data { id: user_id, .. }): LoggedInUser,
        ParamId(param_folder_id): ParamId,
    ) -> WebResult {
        /*
            We use the get_folder_by_user_id to get the folder that the user owns / accessible to
            This function will fail if the param_folder_id points to a folder that
            the user does not own or accessible to
        */
        let target = folder_service
            .get_folder_by_user_id(vec![folder::id::equals(param_folder_id)], user_id)
            .await?;

        /*
            Check the status of the target folder
            If the parent of the target folder is None
            means that the target folder is a root folder of ours or some other user
        */
        if target.parent_folder_id.is_none() {
            return Err(Error::Forbidden);
        }

        let deleted_folder = folder_service.delete_folder(target.id).await?;

        for id in folder_service
            .get_nested_files_from_folder(deleted_folder.id)
            .await?
        {
            storage.delete_file(&id).await?;
            storage.delete_folder(&format!("{id}/")).await?;
        }

        Ok(Web::ok("Folder deleted successfully", ()))
    }
    Router::new().route("/delete/:folder_id", delete(delete_folder_handler))
}
