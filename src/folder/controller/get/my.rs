use axum::{extract::State, routing::get, Router};

use crate::{
    folder::request::query::FolderQuery,
    prisma::{folder, user},
    user::request::loggedin::LoggedInUser,
    web::Web,
    GlobalState, WebResult,
};

// In this route, the owner_id is discarded, and default to the login user
// EVEN IF the user provides user_id in the query, the code will still work
// but the filter will not be applied
pub fn get_my_folders() -> Router<GlobalState> {
    async fn my_folders_handler(
        State(GlobalState { db, .. }): State<GlobalState>,
        LoggedInUser(user::Data { id: user_id, .. }): LoggedInUser,
        FolderQuery {
            id,
            parent: parent_folder_id,
            folder_name,
            visibility,
            created_at,
            updated_at,
            ..
        }: FolderQuery,
    ) -> WebResult {
        let mut filters = vec![folder::owner_id::equals(user_id)];

        if let Some(id) = id {
            filters.push(folder::id::equals(id))
        }

        if let Some(folder_name) = folder_name {
            filters.push(folder::folder_name::equals(folder_name))
        }

        if let Some(visibility) = visibility {
            filters.push(folder::visibility::equals(visibility))
        }

        if let Some(created_at) = created_at {
            filters.push(folder::created_at::equals(created_at))
        }

        if let Some(updated_at) = updated_at {
            filters.push(folder::updated_at::equals(updated_at))
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
