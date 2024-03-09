use axum::{extract::State, routing::get, Router};

use crate::{
	file::model::query::FileQuery,
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
		State(GlobalState { file_service, .. }): State<GlobalState>,
		LoggedInUser(UserSelect { id: user_id, .. }): LoggedInUser,
		FileQuery {
			id,
			parent_folder_id,
			filename,
			extension,
			visibility,
			created_at,
			updated_at,
			..
		}: FileQuery,
	) -> WebResult {
		let my_files = file_service
			.get_child_files_from_folders(
				id,
				Some(user_id),
				parent_folder_id,
				filename,
				extension,
				visibility,
				created_at,
				updated_at,
			)
			.await?;

		Ok(Web::ok("Get all personal files successfully", my_files))
	}
	Router::new().route("/my", get(get_my_files_handler))
}
