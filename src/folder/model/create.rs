use axum::{async_trait, body::Body, extract::FromRequest, http::Request, Json};
use serde::Deserialize;
use validator::Validate;

use crate::{error::Error, prisma::Visibility, validation::uuid::check_uuid, GlobalState};

use super::validation::check_folder_name;

#[derive(Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct CreateFolderRequest {
    #[validate(custom = "check_uuid")]
    pub parent: Option<String>,
    #[validate(custom = "check_folder_name")]
    pub folder_name: String,
    pub visibility: Option<Visibility>,
}

#[async_trait]
impl FromRequest<GlobalState, Body> for CreateFolderRequest {
    type Rejection = Error;
    async fn from_request(
        req: Request<Body>,
        state: &GlobalState,
    ) -> Result<Self, Self::Rejection> {
        let Json(body) = Json::<CreateFolderRequest>::from_request(req, state).await?;
        body.validate()?;
        Ok(body)
    }
}
