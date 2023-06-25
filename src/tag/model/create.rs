use axum::{async_trait, body::Body, extract::FromRequest, http::Request, Json};
use serde::Deserialize;
use validator::Validate;

use crate::{error::Error, GlobalState};

use super::validation::check_tag_name;

#[derive(Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct CreateTagRequest {
    #[validate(custom = "check_tag_name")]
    pub tag_name: String,
}

#[async_trait]
impl FromRequest<GlobalState, Body> for CreateTagRequest {
    type Rejection = Error;
    async fn from_request(
        req: Request<Body>,
        state: &GlobalState,
    ) -> Result<Self, Self::Rejection> {
        let Json(req) = Json::<CreateTagRequest>::from_request(req, state).await?;
        Ok(req)
    }
}
