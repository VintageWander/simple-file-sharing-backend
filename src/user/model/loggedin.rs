use axum::{async_trait, extract::FromRequestParts, http::request::Parts};
use axum_extra::extract::CookieJar;

use crate::{auth::utils::decode::decode_access_token, error::Error, GlobalState};

use super::select::{UserSelect, UserSelectWithPassword};

pub struct LoggedInUser(pub UserSelect);

#[async_trait]
impl FromRequestParts<GlobalState> for LoggedInUser {
	type Rejection = Error;
	async fn from_request_parts(
		parts: &mut Parts,
		state: &GlobalState,
	) -> Result<Self, Self::Rejection> {
		let cookies = CookieJar::from_request_parts(parts, state)
			.await
			.expect("This should not be happening");

		let access_token = cookies
			.get("accessToken")
			.ok_or_else(|| Error::Unauthorized)?
			.value()
			.to_string();

		let user_id = decode_access_token(access_token)?;
		let user = state.user_service.get_user_by_id(user_id).await?;
		Ok(Self(user))
	}
}

pub struct LoggedInUserWithPassword(pub UserSelectWithPassword);

#[async_trait]
impl FromRequestParts<GlobalState> for LoggedInUserWithPassword {
	type Rejection = Error;
	async fn from_request_parts(
		parts: &mut Parts,
		state: &GlobalState,
	) -> Result<Self, Self::Rejection> {
		let cookies = CookieJar::from_request_parts(parts, state)
			.await
			.expect("This should not be happening");

		let access_token = cookies
			.get("accessToken")
			.ok_or_else(|| Error::Unauthorized)?
			.value()
			.to_string();

		let user_id = decode_access_token(access_token)?;
		let user = state
			.user_service
			.get_user_by_id_with_password(user_id)
			.await?;
		Ok(Self(user))
	}
}
