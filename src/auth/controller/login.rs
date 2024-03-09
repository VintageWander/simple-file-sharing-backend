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
	prisma::user,
	web::Web,
	GlobalState, WebResult,
};

pub fn login() -> Router<GlobalState> {
	async fn login_handler(
		State(GlobalState { user_service, .. }): State<GlobalState>,
		cookies: CookieJar,
		LoginRequest { username, password }: LoginRequest,
	) -> WebResult {
		let user = user_service
			.get_user_by_login_info(username, password)
			.await?;

		let (access_token, refresh_token) =
			(encode_access_token(&user)?, encode_refresh_token(&user)?);

		let (access_cookie, refresh_cookie) = (
			make_access_cookie(access_token),
			make_refresh_cookie(refresh_token.clone()),
		);

		let updated_user = user_service
			.update_user(user.id, vec![user::refresh_token::set(refresh_token)])
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
