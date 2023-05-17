use axum::Router;

use crate::Database;

pub mod create;
pub mod delete;
pub mod get;
pub mod update;

pub struct FolderController {}
impl FolderController {
    pub fn routes() -> Router<Database> {
        let controller = FolderController {};
        Router::new().nest(
            "/folder",
            Router::new()
                // /folder?parentFolderId={folder id}
                .merge(controller.get_folders())
                // /folder/create
                .merge(controller.create_folder())
                .merge(controller.update_folder())
                .merge(controller.delete_folder()),
        )
    }
}
