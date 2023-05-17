use axum::{async_trait, body::Body, extract::FromRequest, http::Request};
use serde::Deserialize;
use validator::Validate;

use crate::{
    error::Error,
    validation::{user::check_password, validation_message},
    Database,
};

#[derive(Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct DeleteUserRequest {
    #[validate(custom = "check_password")]
    pub password: String,

    #[validate(custom = "check_password")]
    pub confirm_password: String,
}

#[async_trait]
impl FromRequest<Database, Body> for DeleteUserRequest {
    type Rejection = Error;
    async fn from_request(req: Request<Body>, state: &Database) -> Result<Self, Self::Rejection> {
        let body = DeleteUserRequest::from_request(req, state).await?;
        let DeleteUserRequest {
            password,
            confirm_password,
        } = &body;
        if password != confirm_password {
            return Err(validation_message("Passwords do not match").into());
        }
        Ok(body)
    }
}
