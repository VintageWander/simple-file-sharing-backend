use axum::{
    async_trait,
    extract::{FromRequestParts, Path},
    http::request::Parts,
};
use uuid::Uuid;

use crate::{error::Error, Database};

pub struct ParamId(pub String);

#[async_trait]
impl FromRequestParts<Database> for ParamId {
    type Rejection = Error;
    async fn from_request_parts(
        parts: &mut Parts,
        state: &Database,
    ) -> Result<Self, Self::Rejection> {
        let Path(id) = Path::<Uuid>::from_request_parts(parts, state).await?;
        Ok(Self(id.to_string()))
    }
}
