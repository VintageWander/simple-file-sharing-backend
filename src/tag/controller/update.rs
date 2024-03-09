use axum::{extract::State, routing::put, Router};

use crate::{
	extractors::param::ParamId,
	tag::model::update::UpdateTagRequest,
	user::model::{loggedin::LoggedInUser, select::UserSelect},
	web::Web,
	GlobalState, WebResult,
};

pub fn update_tag() -> Router<GlobalState> {
	async fn update_tag_handler(
		State(GlobalState { tag_service, .. }): State<GlobalState>,
		LoggedInUser(UserSelect { id: user_id, .. }): LoggedInUser,
		ParamId(tag_id): ParamId,
		UpdateTagRequest { tag_name }: UpdateTagRequest,
	) -> WebResult {
		let old_tag = tag_service.get_owned_tag(tag_id, user_id).await?;
		let updated_tag = tag_service.update_tag(old_tag.id, tag_name).await?;
		Ok(Web::ok("Update tag successfully", updated_tag))
	}
	Router::new().route("/update/:tag_id", put(update_tag_handler))
}
