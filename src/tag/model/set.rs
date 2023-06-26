use std::collections::HashSet;

use axum::{async_trait, body::Body, extract::FromRequest, http::Request, Json};
use serde::Deserialize;
use validator::Validate;

use crate::{error::Error, validation::uuid::check_uuid_set, GlobalState};

#[derive(Deserialize, Validate)]
pub struct SetTagRequest {
    #[validate(custom = "check_uuid_set")]
    pub tag_names: HashSet<String>,

    #[validate(custom = "check_uuid_set")]
    pub file_ids: HashSet<String>,

    #[validate(custom = "check_uuid_set")]
    pub folder_ids: HashSet<String>,
}

#[async_trait]
impl FromRequest<GlobalState, Body> for SetTagRequest {
    type Rejection = Error;
    async fn from_request(
        req: Request<Body>,
        state: &GlobalState,
    ) -> Result<Self, Self::Rejection> {
        let Json(req) = Json::<SetTagRequest>::from_request(req, state).await?;

        req.validate()?;

        Ok(req)
    }
}
