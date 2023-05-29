use axum::{extract::State, routing::get, Router};

use crate::{
    file::{request::query::FileQuery, response::file_response},
    prisma::{file, user, Visibility},
    user::request::loggedin::LoggedInUser,
    web::Web,
    GlobalState, WebResult,
};

// On the handlers side
// We only have to deal with owner_id, parent, and visiblity

pub fn get_shared_files() -> Router<GlobalState> {
    async fn get_shared_files_handler(
        State(GlobalState { db, .. }): State<GlobalState>,
        LoggedInUser(user::Data { id: user_id, .. }): LoggedInUser,
        FileQuery {
            parent,
            mut filters,
            ..
        }: FileQuery,
    ) -> WebResult {
        filters.extend(vec![
            file::visibility::equals(Visibility::Shared),
            file::collaborators::some(vec![user::id::equals(user_id)]),
        ]);

        let shared_files = db
            .file()
            .find_many(filters)
            .select(file_response::select())
            .exec()
            .await?;

        Ok(Web::ok(
            "Get all shared to me files successfully",
            shared_files,
        ))
    }
    Router::new().route("/shared", get(get_shared_files_handler))
}
