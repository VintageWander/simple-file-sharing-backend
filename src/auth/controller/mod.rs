pub mod login;
pub mod logout;
pub mod refresh;

use axum::Router;

use crate::Database;

#[derive(Clone)]
pub struct AuthController {}
impl AuthController {
    pub fn routes() -> Router<Database> {
        let controller = AuthController {};
        Router::new().nest(
            "/auth",
            Router::new()
                .merge(controller.login())
                .merge(controller.logout())
                .merge(controller.refresh()),
        )
    }
}
