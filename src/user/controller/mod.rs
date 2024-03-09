use axum::Router;

use crate::GlobalState;

use self::{
	create::create_user,
	delete::delete_user,
	get::{get_user, get_users},
	profile::profile,
	update::update_user,
};

pub mod create;
pub mod delete;
pub mod get;
pub mod profile;
pub mod update;

pub fn user_routes() -> Router<GlobalState> {
	Router::new().nest(
		"/user",
		Router::new()
			// GET /user
			.merge(get_users())
			// GET /user
			.merge(get_user())
			// GET /user/profile
			.merge(profile())
			// POST /user/create
			.merge(create_user())
			// PUT /user/update
			.merge(update_user())
			// DELETE /user/delete
			.merge(delete_user()),
	)
}
