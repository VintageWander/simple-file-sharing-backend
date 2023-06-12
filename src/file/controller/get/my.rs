use axum::{extract::State, routing::get, Router};

use crate::{
    file::model::query::FileQuery,
    prisma::file,
    user::model::{loggedin::LoggedInUser, select::UserSelect},
    web::Web,
    GlobalState, WebResult,
};

/*
    On the handlers side

    We only have to deal with owner_id, parent, and visiblity
*/

pub fn get_my_files() -> Router<GlobalState> {
    async fn get_my_files_handler(
        State(GlobalState {
            db, file_service, ..
        }): State<GlobalState>,
        LoggedInUser(UserSelect { id: user_id, .. }): LoggedInUser,
        FileQuery {
            parent_folder_id,
            visibility,
            mut filters,
            ..
        }: FileQuery,
    ) -> WebResult {
        filters.push(file::owner_id::equals(user_id));

        if let Some(visibility) = visibility {
            filters.push(file::visibility::equals(visibility))
        }

        let my_files = file_service
            .get_child_files_from_folders(parent_folder_id, filters)
            .await?;

        Ok(Web::ok("Get all personal files successfully", my_files))
    }
    Router::new().route("/my", get(get_my_files_handler))
}
