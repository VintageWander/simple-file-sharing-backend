use axum::{
	extract::{Path, State},
	routing::put,
	Router,
};

use crate::{
	extractors::param::ParamId,
	file_version::model::FileVersionSelect,
	prisma::file,
	user::model::{loggedin::LoggedInUser, select::UserSelect},
	web::Web,
	GlobalState, WebResult,
};

pub fn restore_file() -> Router<GlobalState> {
	async fn restore_file_handler(
		State(GlobalState {
			file_service,
			file_version_service,
			storage,
			..
		}): State<GlobalState>,
		ParamId(file_id): ParamId,
		Path(version_number): Path<i64>,
		LoggedInUser(UserSelect { id: user_id, .. }): LoggedInUser,
	) -> WebResult {
		let target_file = file_service
			.get_file_by_user_id(vec![file::id::equals(file_id)], user_id)
			.await?;

		let FileVersionSelect {
			file: target_file,
			version_number: target_version,
		} = file_version_service
			.get_version_by_file_id(target_file.id, version_number)
			.await?;

		let FileVersionSelect {
			file: new_file,
			version_number: new_version,
		} = file_version_service
			.create_version_for_file(target_file.id.clone())
			.await?;

		// Move the current file to backup location
		storage
			.move_file(
				&format!("{}.{}", new_file.id, new_file.extension.to_string()),
				&format!(
					"{}/{}.{}",
					new_file.id,
					new_version,
					new_file.extension.to_string()
				),
			)
			.await?;

		// Move the file from backup location to its original location
		// (Restoring the file)
		storage
			.move_file(
				&format!(
					"{}/{}.{}",
					target_file.id,
					target_version,
					target_file.extension.to_string()
				),
				&format!("{}.{}", target_file.id, target_file.extension.to_string()),
			)
			.await?;

		Ok(Web::ok("Restored file successfully", ()))
	}
	Router::new().route(
		"/restore/:file_id/:version_number",
		put(restore_file_handler),
	)
}
