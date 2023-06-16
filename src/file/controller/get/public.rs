use axum::{extract::State, routing::get, Router};

use crate::{file::model::query::FileQuery, prisma::Visibility, web::Web, GlobalState, WebResult};

/*
    PUBLIC route ignores visibility field

    On the handlers side
    We only have to deal with owner_id, parent, and visiblity
*/

pub fn get_public_files() -> Router<GlobalState> {
    async fn get_public_files_handler(
        State(GlobalState { file_service, .. }): State<GlobalState>,
        FileQuery {
            id,
            owner_id,
            parent_folder_id,
            filename,
            extension,
            created_at,
            updated_at,
            ..
        }: FileQuery,
    ) -> WebResult {
        let public_files = file_service
            .get_child_files_from_folders(
                id,
                owner_id,
                parent_folder_id,
                filename,
                extension,
                Some(Visibility::Public),
                created_at,
                updated_at,
            )
            .await?;
        Ok(Web::ok("Get all public files success", public_files))
    }
    Router::new().route("/public", get(get_public_files_handler))
}
