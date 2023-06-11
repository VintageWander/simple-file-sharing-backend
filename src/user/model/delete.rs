use axum::{async_trait, body::Body, extract::FromRequest, http::Request, Json};
use serde::Deserialize;
use validator::Validate;

use crate::{error::Error, validation::validation_message, GlobalState};

use super::validation::check_password;

#[derive(Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct DeleteUserRequest {
    #[validate(custom = "check_password")]
    pub password: String,

    #[validate(custom = "check_password")]
    pub confirm_password: String,
}

#[async_trait]
impl FromRequest<GlobalState, Body> for DeleteUserRequest {
    type Rejection = Error;
    async fn from_request(
        req: Request<Body>,
        state: &GlobalState,
    ) -> Result<Self, Self::Rejection> {
        let Json(body) = Json::<DeleteUserRequest>::from_request(req, state).await?;
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
