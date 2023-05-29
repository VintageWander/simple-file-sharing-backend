use axum::{extract::State, routing::get, Router};

use crate::{GlobalState, WebResult};

pub fn get_public_files() -> Router<GlobalState> {
    async fn get_public_files_handler(
        State(GlobalState { db, .. }): State<GlobalState>,
    ) -> WebResult {
        todo!()
    }
    Router::new().route("/public", get(get_public_files_handler))
}
