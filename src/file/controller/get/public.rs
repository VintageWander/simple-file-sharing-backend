use axum::{extract::State, routing::get, Router};

use crate::{
    file::request::query::FileQuery,
    prisma::{file, folder, Visibility},
    web::Web,
    GlobalState, WebResult,
};

/*
    PUBLIC route ignores visibility field

    On the handlers side
    We only have to deal with owner_id, parent, and visiblity
*/

pub fn get_public_files() -> Router<GlobalState> {
    async fn get_public_files_handler(
        State(GlobalState { db, .. }): State<GlobalState>,
        FileQuery {
            owner_id,
            parent,
            mut filters,
            ..
        }: FileQuery,
    ) -> WebResult {
        filters.push(file::visibility::equals(Visibility::Public));

        if let Some(owner_id) = owner_id {
            filters.push(file::owner_id::equals(owner_id))
        }

        let starting_point = match parent {
            Some(parent) => folder::id::equals(parent),
            None => folder::parent_folder_id::equals(None),
        };

        let public_files: Vec<_> = db
            .folder()
            .find_many(vec![starting_point])
            .select(folder::select!({
                child_files(filters): select {
                    id
                    owner: select {
                        id username email created_at updated_at
                    }
                    parent_folder_id
                    filename
                    extension
                    visibility
                    created_at
                    updated_at
                }
            }))
            .exec()
            .await?
            .into_iter()
            .flat_map(|root_folder| root_folder.child_files)
            .collect();

        Ok(Web::ok("Get all public files success", public_files))
    }
    Router::new().route("/public", get(get_public_files_handler))
}
