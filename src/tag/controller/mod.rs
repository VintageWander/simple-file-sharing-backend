use axum::Router;

use crate::GlobalState;

use self::{create::create_tag, delete::delete_tag, get::get_tags, set::set, update::update_tag};

pub mod create;
pub mod delete;
pub mod get;
pub mod set;
pub mod update;

pub fn tag_route() -> Router<GlobalState> {
	Router::new().nest(
		"/tag",
		Router::new()
			// GET /tag
			.merge(get_tags())
			// POST /tag/create
			.merge(create_tag())
			// POST /tag/update/:tag_id
			.merge(update_tag())
			// DELETE /tag/delete/:tag_id
			.merge(delete_tag())
			// PUT /tag/set
			.merge(set()),
	)
}
