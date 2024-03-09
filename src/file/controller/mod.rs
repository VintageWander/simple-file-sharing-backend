pub mod collab;
pub mod content;
pub mod create;
pub mod delete;
pub mod get;
pub mod restore;
pub mod update;

use axum::Router;

use crate::GlobalState;

use self::{
	collab::set_file_collaborators,
	content::{generate_file_temp_key, get_file_content},
	create::create_file,
	delete::{delete_file, delete_file_version},
	get::{my::get_my_files, public::get_public_files, shared::get_shared_files},
	restore::restore_file,
	update::update_file,
};

pub fn file_routes() -> Router<GlobalState> {
	Router::new().nest(
		"/file",
		Router::new()
			// GET /file/public?query
			.merge(get_public_files())
			// GET /file/my?query
			.merge(get_my_files())
			// GET /file/shared?query
			.merge(get_shared_files())
			// GET /file/create
			.merge(create_file())
			// POST /file/update/:file_id
			.merge(update_file())
			// PUT /file/delete/:file_id
			.merge(delete_file())
			// DELETE /file/delete/:file_id/:version_number
			.merge(delete_file_version())
			// PUT /file/restore/:file_id/:version_number
			.merge(restore_file())
			// PUT /file/collaborators
			.merge(set_file_collaborators())
			// GET /file/content/:file_id
			.merge(get_file_content())
			// POST /file/content/:file_id
			.merge(generate_file_temp_key()),
	)
}
