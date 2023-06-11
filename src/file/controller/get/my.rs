use axum::{extract::State, routing::get, Router};

use crate::{
    file::model::{query::FileQuery, select::child_files_select},
    prisma::{file, folder},
    user::model::{loggedin::LoggedInUser, select::UserSelect},
    web::Web,
    GlobalState, WebResult,
};

/*
    On the handlers side

    We only have to deal with owner_id, parent, and visiblity
*/

pub fn get_my_files() -> Router<GlobalState> {
    async fn get_my_files_handler(
        State(GlobalState { db, .. }): State<GlobalState>,
        LoggedInUser(UserSelect { id: user_id, .. }): LoggedInUser,
        FileQuery {
            parent,
            visibility,
            mut filters,
            ..
        }: FileQuery,
    ) -> WebResult {
        filters.push(file::owner_id::equals(user_id));

        if let Some(visibility) = visibility {
            filters.push(file::visibility::equals(visibility))
        }

        let starting_point = match parent {
            Some(parent) => folder::id::equals(parent),
            None => folder::parent_folder_id::equals(None),
        };

        let my_files: Vec<_> = db
            .folder()
            .find_many(vec![starting_point])
            .select(child_files_select::select(filters))
            .exec()
            .await?
            .into_iter()
            .flat_map(|root_folder| root_folder.child_files)
            .collect();

        Ok(Web::ok("Get all personal files successfully", my_files))
    }
    Router::new().route("/my", get(get_my_files_handler))
}
