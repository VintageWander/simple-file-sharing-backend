use axum::{extract::State, routing::get, Router};

use crate::{
	folder::model::query::FolderQuery,
	user::model::{loggedin::LoggedInUser, select::UserSelect},
	web::Web,
	GlobalState, WebResult,
};
/*
	In this route, the owner_id is discarded, and default to the login user

	EVEN IF the user provides user_id in the query, the code will still work
	but the filter will not be applied

	Personal route IGNORES owner_id

	On the handlers side
	We only have to deal with owner_id, parent, and visiblity
*/

pub fn get_my_folders() -> Router<GlobalState> {
	async fn my_folders_handler(
		State(GlobalState { folder_service, .. }): State<GlobalState>,
		LoggedInUser(UserSelect { id: user_id, .. }): LoggedInUser,
		FolderQuery {
			id,
			parent_folder_id,
			folder_name,
			visibility,
			created_at,
			updated_at,
			..
		}: FolderQuery,
	) -> WebResult {
		let my_folders = folder_service
			.get_child_folders_from_folders(
				id,
				Some(user_id),
				parent_folder_id,
				folder_name,
				visibility,
				created_at,
				updated_at,
			)
			.await?;

		Ok(Web::ok("Get all personal folders successfully", my_folders))
	}
	Router::new().route("/my", get(my_folders_handler))
}
