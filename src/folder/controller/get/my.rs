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
        State(GlobalState { db, .. }): State<GlobalState>,
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

        let starting_point = match parent_folder_id {
            Some(parent_id) => folder::id::equals(parent_id),
            None => folder::parent_folder_id::equals(None),
        };

        let my_folders: Vec<_> = db
            .folder()
            .find_many(vec![starting_point])
            .select(folder::select!({
                child_folders(filters): select {
                    id
                    owner: select {
                        id username email created_at updated_at
                    }
                    parent_folder_id
                    folder_name
                    visibility
                    created_at
                    updated_at
                }
            }))
            .exec()
            .await?
            .into_iter()
            .flat_map(|root_folder| root_folder.child_folders)
            .collect();

        Ok(Web::ok("Get all personal folders successfully", my_folders))
    }
    Router::new().route("/my", get(my_folders_handler))
}
