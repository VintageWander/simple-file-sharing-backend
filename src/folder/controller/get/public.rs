use axum::{
    extract::{Query, State},
    routing::get,
    Router,
};
use serde::Deserialize;

use crate::{
    folder::{request::query::FolderQuery, response::folder_response},
    prisma::folder,
    web::Web,
    GlobalState, WebResult,
};

// This route will discard visibility search in the query string
// And sets visibility to Public
pub fn get_public_folders() -> Router<GlobalState> {
    // This function does not list all folders that exists in the database
    // But rather lists all folders in the root directory
    async fn get_public_folders_handler(
        State(GlobalState { db, .. }): State<GlobalState>,
        FolderQuery {
            id,
            owner_id,
            parent_folder_id,
            folder_name,
            created_at,
            updated_at,
            ..
        }: FolderQuery,
    ) -> WebResult {
        let mut filters = vec![];

        if let Some(id) = id {
            filters.push(folder::id::equals(id))
        }

        if let Some(owner_id) = owner_id {
            filters.push(folder::owner_id::equals(owner_id))
        }

        if let Some(folder_name) = folder_name {
            filters.push(folder::folder_name::equals(folder_name))
        }

        if let Some(created_at) = created_at {
            filters.push(folder::created_at::equals(created_at))
        }

        if let Some(updated_at) = updated_at {
            filters.push(folder::updated_at::equals(updated_at))
        }

        // If the parent folder id is provided, then that should be a starting point folder
        // to search forward
        // Else just picks all root folders from the system
        let starting_point = match parent_folder_id {
            Some(parent_id) => folder::id::equals(parent_id),
            None => folder::parent_folder_id::equals(None),
        };

        let folders: Vec<_> = db
            .folder()
            .find_many(vec![starting_point])
            .select(folder::select!({ child_folders(filters) }))
            .exec()
            .await?
            .into_iter()
            .flat_map(|root_folder| root_folder.child_folders)
            .collect();

        Ok(Web::ok("Get all folders successfully", folders))
    }
    Router::new().route("/public", get(get_public_folders_handler))
}
