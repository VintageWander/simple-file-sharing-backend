use axum::{async_trait, body::Body, extract::FromRequest, http::Request};
use axum_typed_multipart::{FieldData, TryFromMultipart, TypedMultipart};
use bytes::Bytes;
use is_empty::IsEmpty;
use validator::Validate;

use crate::{error::Error, prisma::Visibility, validation::uuid::check_uuid, GlobalState};

#[derive(TryFromMultipart, Validate, IsEmpty)]
pub struct UpdateFileRequest {
    #[validate(custom = "check_uuid")]
    pub parent: Option<String>,

    pub visibility: Option<Visibility>,

    pub file: Option<FieldData<Bytes>>,
}

#[async_trait]
impl FromRequest<GlobalState, Body> for UpdateFileRequest {
    type Rejection = Error;
    async fn from_request(
        req: Request<Body>,
        state: &GlobalState,
    ) -> Result<Self, Self::Rejection> {
        let TypedMultipart(body) =
            TypedMultipart::<UpdateFileRequest>::from_request(req, state).await?;
        if body.is_empty() {
            return Err(Error::NoContent);
        }
        body.validate()?;
        Ok(body)
    }
}
