use axum::{extract::State, routing::put, Router};

use crate::{
	file::model::collab::SetFileCollabRequest,
	prisma::file,
	user::model::{loggedin::LoggedInUser, select::UserSelect},
	web::Web,
	GlobalState, WebResult,
};

pub fn set_file_collaborators() -> Router<GlobalState> {
	async fn set_file_collaborators_handler(
		State(GlobalState {
			file_service,
			user_service,
			..
		}): State<GlobalState>,
		LoggedInUser(UserSelect { id: user_id, .. }): LoggedInUser,
		SetFileCollabRequest { file_id, user_ids }: SetFileCollabRequest,
	) -> WebResult {
		let target_file = file_service
			.get_file_by_user_id(vec![file::id::equals(file_id)], user_id)
			.await?;

		let mut collaborators = vec![];
		for id in user_ids {
			collaborators.push(user_service.get_user_by_id(id).await?)
		}

		file_service
			.set_collaborators_to_file(target_file.id, collaborators)
			.await?;

		Ok(Web::ok("Set collaborators to file successfully", ()))
	}
	Router::new().route("/collaborators", put(set_file_collaborators_handler))
}
