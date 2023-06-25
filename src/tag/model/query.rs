use axum::{
    async_trait,
    extract::{FromRequestParts, Query},
    http::request::Parts,
};
use serde::Deserialize;
use validator::Validate;

use crate::{error::Error, validation::uuid::check_uuid, GlobalState};

#[derive(Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct TagQuery {
    #[validate(custom = "check_uuid")]
    pub id: Option<String>,

    pub tag_name: Option<String>,

    #[validate(custom = "check_uuid")]
    pub owner_id: Option<String>,

    #[validate(custom = "check_uuid")]
    pub file_id: Option<String>,

    #[validate(custom = "check_uuid")]
    pub folder_id: Option<String>,
}

#[async_trait]
impl FromRequestParts<GlobalState> for TagQuery {
    type Rejection = Error;
    async fn from_request_parts(
        parts: &mut Parts,
        state: &GlobalState,
    ) -> Result<Self, Self::Rejection> {
        let Query(query) = Query::<TagQuery>::from_request_parts(parts, state).await?;

        query.validate()?;

        Ok(query)
    }
}
