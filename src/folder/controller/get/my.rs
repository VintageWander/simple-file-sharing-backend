use axum::{extract::State, routing::get, Router};

use crate::{
    folder::request::query::FolderQuery,
    prisma::folder,
    user::{request::loggedin::LoggedInUser, response::user_response::Data},
    web::Web,
    GlobalState, WebResult,
};

// In this route, the owner_id is discarded, and default to the login user
// EVEN IF the user provides user_id in the query, the code will still work
// but the filter will not be applied

// Personal route ignores owner_id

// On the handlers side
// We only have to deal with owner_id, parent, and visiblity

pub fn get_my_folders() -> Router<GlobalState> {
    async fn my_folders_handler(
        State(GlobalState {
            db, folder_service, ..
        }): State<GlobalState>,
        LoggedInUser(Data { id: user_id, .. }): LoggedInUser,
        FolderQuery {
            parent: parent_folder_id,
            visibility,
            mut filters,
            ..
        }: FolderQuery,
    ) -> WebResult {
        filters.push(folder::owner_id::equals(user_id));

        if let Some(visibility) = visibility {
            filters.push(folder::visibility::equals(visibility))
        }

        let my_folders = folder_service
            .get_child_folders_from_folders(parent_folder_id, filters)
            .await?;

        Ok(Web::ok("Get all personal folders successfully", my_folders))
    }
    Router::new().route("/my", get(my_folders_handler))
}
