pub mod get;

use axum::Router;

use crate::Database;

pub struct FileController {}
impl FileController {
    pub fn routes() -> Router<Database> {
        let controller = FileController {};
        Router::new().nest("/file", Router::new().merge(controller.get_files()))
    }
}
