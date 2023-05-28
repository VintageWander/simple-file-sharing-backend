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
pub struct CreateUserRequest {
    #[validate(custom = "check_username")]
    pub username: String,

    #[validate(email(message = "Invalid email form"))]
    pub email: String,

    #[validate(custom = "check_password")]
    pub password: String,

    #[validate(custom = "check_password")]
    pub confirm_password: String,
}

#[async_trait]
impl FromRequest<GlobalState, Body> for CreateUserRequest {
    type Rejection = Error;
    async fn from_request(
        req: Request<Body>,
        state: &GlobalState,
    ) -> Result<Self, Self::Rejection> {
        let Json(body) = Json::<CreateUserRequest>::from_request(req, state).await?;
        let CreateUserRequest {
            password,
            confirm_password,
            ..
        } = &body;
        if password != confirm_password {
            return Err(validation_message("Passwords are not equal").into());
        }
        Ok(body)
    }
}
