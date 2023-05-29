use axum::{extract::State, routing::get, Router};

use crate::{
    folder::request::query::FolderQuery,
    prisma::{folder, user, Visibility},
    user::request::loggedin::LoggedInUser,
    web::Web,
    GlobalState, WebResult,
};

// When it comes to getting all "shared to me folders"
// both the owner id and visibility are discarded
pub fn get_shared_folders() -> Router<GlobalState> {
    async fn get_shared_folders_handler(
        State(GlobalState { db, .. }): State<GlobalState>,
        LoggedInUser(user::Data { id: user_id, .. }): LoggedInUser,
        FolderQuery {
            id,
            parent: parent_folder_id,
            folder_name,
            created_at,
            updated_at,
            ..
        }: FolderQuery,
    ) -> WebResult {
        let mut filters = vec![
            folder::visibility::equals(Visibility::Shared),
            folder::collaborators::some(vec![user::id::equals(user_id)]),
        ];

        if let Some(folder_id) = id {
            filters.push(folder::id::equals(folder_id))
        };

        if let Some(folder_name) = folder_name {
            filters.push(folder::folder_name::equals(folder_name))
        }

        if let Some(created_at) = created_at {
            filters.push(folder::created_at::equals(created_at))
        }

        if let Some(updated_at) = updated_at {
            filters.push(folder::updated_at::equals(updated_at))
        }

        let shared_folders = db
            .folder()
            .find_many(filters)
            .select(folder::select!({
                id
                owner: select {
                    id username email created_at updated_at
                }
                collaborators
                parent_folder_id
                folder_name
                visibility
                created_at
                updated_at
            }))
            .exec()
            .await?;

        Ok(Web::ok(
            "Get all shared to me folders success",
            shared_folders,
        ))
    }
    Router::new().route("/shared", get(get_shared_folders_handler))
}
