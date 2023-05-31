use axum::{extract::State, routing::post, Router};

use crate::{
    error::Error,
    folder::{request::create::CreateFolderRequest, response::folder_response},
    prisma::{folder, Visibility},
    user::{request::loggedin::LoggedInUser, response::user_response::Data},
    web::Web,
    GlobalState, WebResult,
};

pub fn create_folder() -> Router<GlobalState> {
    async fn create_folder_handler(
        State(GlobalState { db, .. }): State<GlobalState>,
        LoggedInUser(Data { id: user_id, .. }): LoggedInUser,
        CreateFolderRequest {
            parent,
            folder_name,
            visibility,
        }: CreateFolderRequest,
    ) -> WebResult {
        // Remember to use create_unchecked

        // First, set the query for starting folder

        let starting_point = match &parent {
            Some(parent) => vec![folder::id::equals(parent.clone())],
            None => vec![
                folder::owner_id::equals(user_id.clone()),
                folder::parent_folder_id::equals(None),
            ],
        };

        // Get the parent folder as a base point to put the new folder in

        let parent_folder = db
            .folder()
            .find_first(starting_point)
            .select(folder::select!({
                id // gets the folder id
                owner_id // gets folder owner
                collaborators: select { // inner join with the collaborators table
                    id // only get the user id, protects user privacy
                }
            }))
            .exec()
            .await?
            .ok_or_else(|| Error::NotFound)?;

        // Prepare the insert query

        let new_folder = db
            .folder()
            .create_unchecked(
                user_id.clone(),
                folder_name,
                visibility.unwrap_or(Visibility::Public),
                vec![folder::parent_folder_id::set(Some(parent_folder.id))],
            )
            .select(folder_response::select())
            .exec();

        // If the parent field wasn't provided in the request body
        // Insert the folder in the root
        if parent.is_none() {
            // Executes
            let new_folder = new_folder.await?;

            // Return
            return Ok(Web::created("Create new folder successfully", new_folder));
        }

        // Ensure the parent folder is ours, OR shared to us
        let proceed = if parent_folder.owner_id == user_id {
            true
        } else {
            parent_folder
                .collaborators
                .into_iter()
                .any(|user| user.id == user_id)
        };

        // We cannot proceed with this action
        if !proceed {
            return Err(Error::Forbidden);
        }

        let new_folder = new_folder.await?;

        // Return
        Ok(Web::created("Create new folder successfully", new_folder))
    }
    Router::new().route("/create", post(create_folder_handler))
}
