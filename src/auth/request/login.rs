use axum::{async_trait, body::Body, extract::FromRequest, http::Request, Json};
use serde::Deserialize;
use validator::Validate;

use crate::{
	error::Error,
	user::model::validation::{check_password, check_username},
	GlobalState,
};

#[derive(Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct LoginRequest {
	#[validate(custom = "check_username")]
	pub username: String,
	#[validate(custom = "check_password")]
	pub password: String,
}

#[async_trait]
impl FromRequest<GlobalState, Body> for LoginRequest {
	type Rejection = Error;
	async fn from_request(
		req: Request<Body>,
		state: &GlobalState,
	) -> Result<Self, Self::Rejection> {
		let Json(body) = Json::<LoginRequest>::from_request(req, state).await?;
		body.validate()?;
		Ok(body)
	}
}
