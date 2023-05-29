use axum::{
    async_trait,
    extract::{FromRequestParts, Query},
    http::request::Parts,
};
use chrono::{DateTime, FixedOffset};
use is_empty::IsEmpty;
use serde::Deserialize;
use validator::Validate;

use crate::{
    error::Error,
    prisma::{Extension, Visibility},
    validation::{file::check_filename, uuid::check_uuid},
    GlobalState,
};

#[derive(Deserialize, Validate, IsEmpty)]
#[serde(rename_all = "camelCase")]
pub struct FileQuery {
    #[validate(custom = "check_uuid")]
    pub id: Option<String>,

    #[validate(custom = "check_uuid")]
    pub owner_id: Option<String>,

    #[validate(custom = "check_uuid")]
    pub parent: Option<String>,

    #[validate(custom = "check_filename")]
    pub filename: Option<String>,

    pub extension: Option<Extension>,

    pub visibility: Option<Visibility>,

    pub created_at: Option<DateTime<FixedOffset>>,

    pub updated_at: Option<DateTime<FixedOffset>>,
}

#[async_trait]
impl FromRequestParts<GlobalState> for FileQuery {
    type Rejection = Error;
    async fn from_request_parts(
        parts: &mut Parts,
        state: &GlobalState,
    ) -> Result<Self, Self::Rejection> {
        let Query(query) = Query::<FileQuery>::from_request_parts(parts, state).await?;

        match query.is_empty() {
            true => Err(Error::NoContent),
            false => {
                query.validate()?;
                Ok(query)
            }
        }
    }
}
