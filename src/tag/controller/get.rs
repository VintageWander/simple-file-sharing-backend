use axum::{extract::State, routing::get, Router};

use crate::{tag::model::query::TagQuery, web::Web, GlobalState, WebResult};

pub fn get_tags() -> Router<GlobalState> {
	async fn get_tags_handler(
		State(GlobalState { tag_service, .. }): State<GlobalState>,
		TagQuery {
			id: tag_id,
			tag_name,
			owner_id,
			file_id,
			folder_id,
		}: TagQuery,
	) -> WebResult {
		let tags = tag_service
			.get_tags(tag_id, tag_name, owner_id, file_id, folder_id)
			.await?;

		Ok(Web::ok("Get all tags success", tags))
	}
	Router::new().route("/", get(get_tags_handler))
}
