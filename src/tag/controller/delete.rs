use axum::{extract::State, routing::delete, Router};

use crate::{
	extractors::param::ParamId,
	user::model::{loggedin::LoggedInUser, select::UserSelect},
	web::Web,
	GlobalState, WebResult,
};

pub fn delete_tag() -> Router<GlobalState> {
	async fn delete_tag_handler(
		State(GlobalState { tag_service, .. }): State<GlobalState>,
		LoggedInUser(UserSelect { id: user_id, .. }): LoggedInUser,
		ParamId(tag_id): ParamId,
	) -> WebResult {
		let owned_tag = tag_service.get_owned_tag(tag_id, user_id).await?;
		tag_service.delete_tag(owned_tag.id).await?;
		Ok(Web::ok("Delete tag successfully", ()))
	}
	Router::new().route("/delete/:tag_id", delete(delete_tag_handler))
}
