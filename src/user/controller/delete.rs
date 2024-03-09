use axum::{extract::State, routing::delete, Router};

use crate::{
	user::model::{
		delete::DeleteUserRequest, loggedin::LoggedInUserWithPassword,
		select::UserSelectWithPassword,
	},
	validation::validation_message,
	web::Web,
	GlobalState, WebResult,
};

pub fn delete_user() -> Router<GlobalState> {
	async fn delete_user_handler(
		State(GlobalState {
			user_service,
			folder_service,
			storage,
			..
		}): State<GlobalState>,
		LoggedInUserWithPassword(UserSelectWithPassword {
			id: user_id,
			password: user_password,
		}): LoggedInUserWithPassword,
		DeleteUserRequest { password, .. }: DeleteUserRequest,
	) -> WebResult {
		if user_password != password {
			return Err(
				validation_message("Password provided does not match current password").into(),
			);
		}

		let root_folder = folder_service.get_root_folder(user_id.clone()).await?;

		for (id, extension) in folder_service
			.get_nested_files_from_folder(root_folder)
			.await?
		{
			storage
				.delete_file(&format!("{}.{}", id, extension.to_string()))
				.await?;
			storage.delete_folder(&format!("{}/", id)).await?;
		}

		user_service.delete_user(user_id).await?;

		Ok(Web::ok("Deleted user successfully", ()))
	}
	Router::new().route("/delete", delete(delete_user_handler))
}
