use axum::{extract::State, http::StatusCode, response::IntoResponse, routing::post, Router};
use axum_extra::extract::CookieJar;

use crate::{
    auth::{
        request::login::LoginRequest,
        utils::{
            cookie::{make_access_cookie, make_refresh_cookie},
            encode::{encode_access_token, encode_refresh_token},
        },
    },
    error::Error,
    prisma::user,
    web::Web,
    Database, WebResult,
};

use super::AuthController;

impl AuthController {
    pub fn login(&self) -> Router<Database> {
        async fn login_handler(
            State(db): State<Database>,
            cookies: CookieJar,
            LoginRequest { username, password }: LoginRequest,
        ) -> WebResult {
            let user = db
                .user()
                .find_first(vec![
                    user::username::equals(username),
                    user::password::equals(password),
                ])
                .exec()
                .await?
                .ok_or_else(|| Error::NotFound)?;

            let (access_token, refresh_token) =
                (encode_access_token(&user)?, encode_refresh_token(&user)?);

            let (access_cookie, refresh_cookie) = (
                make_access_cookie(access_token),
                make_refresh_cookie(refresh_token.clone()),
            );

            let updated_user = db
                .user()
                .update(
                    user::id::equals(user.id),
                    vec![user::refresh_token::set(refresh_token)],
                )
                .select(user::select!({
                    id username email created_at updated_at
                }))
                .exec()
                .await?;

            let response = (
                StatusCode::OK,
                cookies.add(access_cookie).add(refresh_cookie),
                Web::ok("Login successfully", updated_user),
            );

            Ok(response.into_response())
        }
        Router::new().route("/login", post(login_handler))
    }
}
