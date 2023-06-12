use axum::{extract::State, routing::get, Router};

use crate::{
    folder::model::query::FolderQuery,
    user::model::{loggedin::LoggedInUser, select::UserSelect},
    web::Web,
    GlobalState, WebResult,
};

/*
    When it comes to getting all "shared to me folders"
    owner_id and visibility are ignored

    On the handlers side
    We only have to deal with owner_id, parent, and visiblity
*/

pub fn get_shared_folders() -> Router<GlobalState> {
    async fn get_shared_folders_handler(
        State(GlobalState {
            db, folder_service, ..
        }): State<GlobalState>,
        LoggedInUser(UserSelect { id: user_id, .. }): LoggedInUser,
        FolderQuery {
            parent_folder_id,
            filters,
            ..
        }: FolderQuery,
    ) -> WebResult {
        let shared_folders = folder_service
            .get_folders_shared_to_user_id(user_id, filters)
            .await?;

        Ok(Web::ok(
            "Get all shared to me folders success",
            shared_folders,
        ))
    }
    Router::new().route("/shared", get(get_shared_folders_handler))
}
