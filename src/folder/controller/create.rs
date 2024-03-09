use axum::{extract::State, routing::post, Router};

use crate::{
	folder::model::create::CreateFolderRequest,
	prisma::folder,
	user::model::{loggedin::LoggedInUser, select::UserSelect},
	web::Web,
	GlobalState, WebResult,
};

pub fn create_folder() -> Router<GlobalState> {
	async fn create_folder_handler(
		State(GlobalState { folder_service, .. }): State<GlobalState>,
		LoggedInUser(UserSelect { id: user_id, .. }): LoggedInUser,
		CreateFolderRequest {
			parent,
			folder_name,
			visibility,
		}: CreateFolderRequest,
	) -> WebResult {
		/*
			Remember to use create_unchecked

			First, set the query for starting folder
		*/

		let starting_point = match &parent {
			Some(parent) => vec![folder::id::equals(parent.clone())],
			None => vec![
				folder::owner_id::equals(user_id.clone()),
				folder::parent_folder_id::equals(None),
			],
		};

		// Get the parent folder as a base point to put the new folder in

		let parent_folder = folder_service
			.get_folder_by_user_id(starting_point, user_id.clone())
			.await?;

		// Create the new folder
		let new_folder = folder_service
			.create_folder(user_id, folder_name, visibility, Some(parent_folder.id))
			.await?;

		// Return
		Ok(Web::created("Create new folder successfully", new_folder))
	}
	Router::new().route("/create", post(create_folder_handler))
}
