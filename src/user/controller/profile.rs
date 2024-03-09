use axum::{extract::State, routing::get, Router};

use crate::{
	user::model::{loggedin::LoggedInUser, select::UserSelect},
	web::Web,
	GlobalState, WebResult,
};

pub fn profile() -> Router<GlobalState> {
	async fn profile_handler(
		State(GlobalState { user_service, .. }): State<GlobalState>,
		LoggedInUser(UserSelect { id: user_id, .. }): LoggedInUser,
	) -> WebResult {
		let filtered_user = user_service.get_user_by_id(user_id).await?;
		Ok(Web::ok("Get user's profile success", filtered_user))
	}
	Router::new().route("/profile", get(profile_handler))
}
