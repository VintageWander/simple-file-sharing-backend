use axum::{async_trait, body::Body, extract::FromRequest, http::Request, Json};
use serde::Deserialize;
use validator::Validate;

use crate::{
    error::Error,
    validation::{user::*, validation_message},
    GlobalState,
};

#[derive(Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct UpdateUserRequest {
    #[validate(custom = "check_username")]
    pub username: Option<String>,

    #[validate(email(message = "Invalid email form"))]
    pub email: Option<String>,

    #[validate(custom = "check_password")]
    pub password: String,

    #[validate(custom = "check_password")]
    pub new_password: Option<String>,

    #[validate(custom = "check_password")]
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
        let UpdateUserRequest {
            new_password,
            confirm_new_password,
            ..
        } = &body;
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
