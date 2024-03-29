use axum::{async_trait, body::Body, extract::FromRequest, http::Request, Json};
use is_empty::IsEmpty;
use serde::Deserialize;
use validator::Validate;

use crate::{error::Error, validation::validation_message, GlobalState};

use super::validation::{check_password, check_password_option, check_username_option};

#[derive(Deserialize, Validate, IsEmpty)]
#[serde(rename_all = "camelCase")]
pub struct UpdateUserRequest {
	#[validate(custom(function = "check_username_option"))]
	pub username: Option<String>,

	#[validate(email(message = "Invalid email form"))]
	pub email: Option<String>,

	#[validate(custom(function = "check_password"))]
	#[is_empty(if = "String::is_empty")]
	pub password: String,

	#[validate(custom(function = "check_password_option"))]
	pub new_password: Option<String>,

	#[validate(custom(function = "check_password_option"))]
	pub confirm_new_password: Option<String>,
}

#[async_trait]
impl FromRequest<GlobalState, Body> for UpdateUserRequest {
	type Rejection = Error;
	async fn from_request(
		req: Request<Body>,
		state: &GlobalState,
	) -> Result<Self, Self::Rejection> {
		let Json(body) = Json::<UpdateUserRequest>::from_request(req, state).await?;

		// Just return no content if the body is empty
		if body.is_empty() {
			return Err(Error::NoContent);
		}

		let UpdateUserRequest {
			new_password,
			confirm_new_password,
			..
		} = &body;

		// Ensure that both new password and confirm new password fields are equal
		if (new_password.is_some() && confirm_new_password.is_none())
			|| (new_password.is_none() && confirm_new_password.is_some())
		{
			return Err(validation_message("Both field newPassword and confirmNewPassword must exists together, or omit them both").into());
		}
		if let (Some(new_password), Some(confirm_new_password)) =
			(new_password, confirm_new_password)
		{
			if new_password != confirm_new_password {
				return Err(validation_message("Both passwords are not the same").into());
			}
		}

		Ok(body)
	}
}
