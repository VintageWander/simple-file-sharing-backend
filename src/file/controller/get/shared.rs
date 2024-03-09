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

pub fn get_shared_files() -> Router<GlobalState> {
	async fn get_shared_files_handler(
		State(GlobalState { file_service, .. }): State<GlobalState>,
		LoggedInUser(UserSelect { id: user_id, .. }): LoggedInUser,
		FileQuery {
			id,
			parent_folder_id,
			filename,
			extension,
			created_at,
			updated_at,
			..
		}: FileQuery,
	) -> WebResult {
		let shared_files = file_service
			.get_files_shared_to_user_id(
				user_id,
				id,
				parent_folder_id,
				filename,
				extension,
				created_at,
				updated_at,
			)
			.await?;

		Ok(Web::ok(
			"Get all shared to me files successfully",
			shared_files,
		))
	}
	Router::new().route("/shared", get(get_shared_files_handler))
}
