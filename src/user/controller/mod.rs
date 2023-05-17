use axum::Router;

use crate::Database;

pub mod create;
pub mod delete;
pub mod get;
pub mod profile;
pub mod update;

#[derive(Clone)]
pub struct UserController {}
impl UserController {
    pub fn routes() -> Router<Database> {
        let controller = UserController {};
        Router::new().nest(
            "/user",
            Router::new()
                // /user
                .merge(controller.get_users())
                // /user
                .merge(controller.get_user())
                // /user/profile
                .merge(controller.profile())
                // /user/create
                .merge(controller.create_user())
                // /user/update
                .merge(controller.update_user())
                // /user/delete
                .merge(controller.delete_user()),
        )
    }
}
