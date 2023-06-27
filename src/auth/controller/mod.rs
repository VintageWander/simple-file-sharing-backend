pub mod login;
pub mod logout;
pub mod refresh;

use axum::Router;
use {login::login, logout::logout, refresh::refresh};

use crate::GlobalState;

pub fn auth_routes() -> Router<GlobalState> {
    Router::new().nest(
        "/auth",
        Router::new()
            // POST /auth/login
            .merge(login())
            // DELETE /auth/logout
            .merge(logout())
            // POST /auth/refresh
            .merge(refresh()),
    )
}
