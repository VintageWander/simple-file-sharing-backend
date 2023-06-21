use axum::{extract::State, http::StatusCode, response::IntoResponse, routing::post, Router};
use axum_extra::extract::CookieJar;

use crate::{
    auth::utils::{
        cookie::make_access_cookie, decode::decode_refresh_token, encode::encode_access_token,
    },
    error::Error,
    web::Web,
    GlobalState, WebResult,
};

pub fn refresh() -> Router<GlobalState> {
    async fn refresh_handler(
        State(GlobalState { user_service, .. }): State<GlobalState>,
        cookies: CookieJar,
    ) -> WebResult {
        let refresh_token = cookies
            .get("refreshToken")
            .ok_or_else(|| Error::MissingRefreshToken)?
            .value()
            .to_string();

        let user_id = decode_refresh_token(refresh_token).map_err(|_| Error::Decode)?;

        let user = user_service.get_user_by_id(user_id).await?;

        let access_token = encode_access_token(&user)?;

        let access_cookie = make_access_cookie(access_token);

        let response = (
            StatusCode::OK,
            cookies.add(access_cookie),
            Web::ok("Refreshed access token successfully", ()),
        );

        Ok(response.into_response())
    }
    Router::new().route("/refresh", post(refresh_handler))
}
