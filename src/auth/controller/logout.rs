use axum::{extract::State, http::StatusCode, response::IntoResponse, routing::delete, Router};
use axum_extra::extract::CookieJar;

use crate::{
    auth::utils::{
        cookie::{make_access_cookie, make_refresh_cookie},
        encode::{encode_access_token, encode_refresh_token},
    },
    prisma::user,
    user::request::loggedin::LoggedInUser,
    web::Web,
    Database, WebResult,
};

use super::AuthController;

impl AuthController {
    pub fn logout(&self) -> Router<Database> {
        async fn logout_handler(
            State(db): State<Database>,
            cookies: CookieJar,
            LoggedInUser(user): LoggedInUser,
        ) -> WebResult {
            let (access_token, refresh_token) =
                (encode_access_token(&user)?, encode_refresh_token(&user)?);

            let (access_cookie, refresh_cookie) = (
                make_access_cookie(access_token),
                make_refresh_cookie(refresh_token),
            );

            db.user()
                .update(
                    user::id::equals(user.id),
                    vec![user::refresh_token::set("".into())],
                )
                .exec()
                .await?;

            let response = (
                StatusCode::OK,
                cookies.remove(access_cookie).remove(refresh_cookie),
                Web::ok("Logout successful", ()),
            );

            Ok(response.into_response())
        }
        Router::new().route("/logout", delete(logout_handler))
    }
}
