use axum::{extract::State, routing::post, Router};

use crate::{
    error::Error,
    folder::{request::create::CreateFolderRequest, response::folder_response},
    prisma::{folder, user::Data, Visibility},
    user::request::loggedin::LoggedInUser,
    web::Web,
    GlobalState, WebResult,
};

pub fn create_folder() -> Router<GlobalState> {
    async fn create_folder_handler(
        State(GlobalState { db, .. }): State<GlobalState>,
        LoggedInUser(Data { id: user_id, .. }): LoggedInUser,
        CreateFolderRequest {
            parent: optional_parent,
            folder_name,
            visibility: optional_visibility,
        }: CreateFolderRequest,
    ) -> WebResult {
        let root_folder = db
            .folder()
            .find_first(vec![
                folder::owner_id::equals(user_id.clone()),
                folder::parent_folder_id::equals(None),
            ])
            .select(folder::select!({ id }))
            .exec()
            .await?
            .ok_or_else(|| Error::NotFound)?;

        let parent = match optional_parent {
            Some(parent) => parent.to_string(),
            None => root_folder.id,
        };

        let visibility = optional_visibility.unwrap_or(Visibility::Private);

        let new_folder = db
            .folder()
            .create_unchecked(
                user_id,
                folder_name,
                visibility,
                vec![folder::parent_folder_id::set(Some(parent))],
            )
            .select(folder_response::select())
            .exec()
            .await?;

        Ok(Web::created("Create new folder successfully", new_folder))
    }
    Router::new().route("/create", post(create_folder_handler))
}
