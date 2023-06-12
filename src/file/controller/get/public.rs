use axum::{extract::State, routing::get, Router};

use crate::{
    file::model::query::FileQuery,
    prisma::{file, Visibility},
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
        State(GlobalState {
            db, file_service, ..
        }): State<GlobalState>,
        FileQuery {
            owner_id,
            parent_folder_id,
            mut filters,
            ..
        }: FileQuery,
    ) -> WebResult {
        filters.push(file::visibility::equals(Visibility::Public));

        if let Some(owner_id) = owner_id {
            filters.push(file::owner_id::equals(owner_id))
        }

        let public_files = file_service
            .get_child_files_from_folders(parent_folder_id, filters)
            .await?;
        Ok(Web::ok("Get all public files success", public_files))
    }
    Router::new().route("/public", get(get_public_files_handler))
}
