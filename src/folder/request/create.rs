use axum::{async_trait, body::Body, extract::FromRequest, http::Request, Json};
use serde::Deserialize;
use uuid::Uuid;
use validator::Validate;

use crate::{error::Error, prisma::Visibility, validation::file::check_folder_name, Database};

#[derive(Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct CreateFolderRequest {
    pub parent: Option<Uuid>,
    #[validate(custom = "check_folder_name")]
    pub folder_name: String,
    pub visibility: Option<Visibility>,
}

#[async_trait]
impl FromRequest<Database, Body> for CreateFolderRequest {
    type Rejection = Error;
    async fn from_request(req: Request<Body>, state: &Database) -> Result<Self, Self::Rejection> {
        let Json(body) = Json::<CreateFolderRequest>::from_request(req, state).await?;
        body.validate()?;
        Ok(body)
    }
}
