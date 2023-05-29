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
    prisma::{file, Extension, Visibility},
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

    #[serde(skip)]
    #[is_empty(if = "Vec::is_empty")]
    pub filters: Vec<file::WhereParam>,
}

#[async_trait]
impl FromRequestParts<GlobalState> for FileQuery {
    type Rejection = Error;
    async fn from_request_parts(
        parts: &mut Parts,
        state: &GlobalState,
    ) -> Result<Self, Self::Rejection> {
        let Query(mut query) = Query::<FileQuery>::from_request_parts(parts, state).await?;

        if query.is_empty() {
            return Err(Error::NoContent);
        }

        let FileQuery {
            id,
            owner_id, // ignored
            parent,   // ignored
            filename,
            extension,
            visibility, // ignored
            created_at,
            updated_at,
            ref mut filters,
        } = &mut query;

        if let Some(id) = id.clone() {
            filters.push(file::id::equals(id))
        }

        if let Some(filename) = filename.clone() {
            filters.push(file::filename::equals(filename))
        }

        if let Some(extension) = extension {
            filters.push(file::extension::equals(*extension))
        }

        if let Some(created_at) = created_at {
            filters.push(file::created_at::equals(*created_at))
        }

        if let Some(updated_at) = updated_at {
            filters.push(file::updated_at::equals(*updated_at))
        }

        Ok(query)
    }
}
