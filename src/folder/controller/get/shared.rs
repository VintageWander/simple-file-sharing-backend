use axum::{extract::State, routing::get, Router};

use crate::{
    folder::{request::query::FolderQuery, response::folder_response},
    prisma::{folder, user, Visibility},
    user::request::loggedin::LoggedInUser,
    web::Web,
    GlobalState, WebResult,
};

// When it comes to getting all "shared to me folders"
// owner_id and visibility are ignored

// On the handlers side
// We only have to deal with owner_id, parent, and visiblity

pub fn get_shared_folders() -> Router<GlobalState> {
    async fn get_shared_folders_handler(
        State(GlobalState { db, .. }): State<GlobalState>,
        LoggedInUser(user::Data { id: user_id, .. }): LoggedInUser,
        FolderQuery {
            parent: parent_folder_id,
            mut filters,
            ..
        }: FolderQuery,
    ) -> WebResult {
        filters.extend(vec![
            folder::visibility::equals(Visibility::Shared),
            folder::collaborators::some(vec![user::id::equals(user_id)]),
        ]);

        let shared_folders = db
            .folder()
            .find_many(filters)
            .select(folder_response::select())
            .exec()
            .await?;

        Ok(Web::ok(
            "Get all shared to me folders success",
            shared_folders,
        ))
    }
    Router::new().route("/shared", get(get_shared_folders_handler))
}
