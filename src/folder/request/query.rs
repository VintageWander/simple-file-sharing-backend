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
    prisma::{folder, Visibility},
    validation::{file::check_folder_name, uuid::check_uuid},
    GlobalState,
};

#[derive(Deserialize, Validate, IsEmpty)]
#[serde(rename_all = "camelCase")]
pub struct FolderQuery {
    #[validate(custom = "check_uuid")]
    pub id: Option<String>,

    #[validate(custom = "check_uuid")]
    pub owner_id: Option<String>,

    #[validate(custom = "check_uuid")]
    pub parent: Option<String>,

    #[validate(custom = "check_folder_name")]
    pub folder_name: Option<String>,

    pub visibility: Option<Visibility>,

    pub created_at: Option<DateTime<FixedOffset>>,

    pub updated_at: Option<DateTime<FixedOffset>>,

    #[serde(skip)]
    #[is_empty(if = "Vec::is_empty")]
    pub filters: Vec<folder::WhereParam>,
}

#[async_trait]
impl FromRequestParts<GlobalState> for FolderQuery {
    type Rejection = Error;
    async fn from_request_parts(
        parts: &mut Parts,
        state: &GlobalState,
    ) -> Result<Self, Self::Rejection> {
        let Query(mut query) = Query::<FolderQuery>::from_request_parts(parts, state).await?;

        if query.is_empty() {
            return Err(Error::NoContent);
        }

        let FolderQuery {
            id,
            owner_id, // ignored
            parent,   // ignored
            folder_name,
            visibility, // ignored
            created_at,
            updated_at,
            ref mut filters,
        } = &mut query;

        /*
            Process common values
            The owner_id, parent, and visibility are left for the handler to process
        */
        if let Some(folder_id) = id.clone() {
            filters.push(folder::id::equals(folder_id))
        };

        if let Some(folder_name) = folder_name.clone() {
            filters.push(folder::folder_name::equals(folder_name))
        }

        if let Some(created_at) = created_at {
            filters.push(folder::created_at::equals(*created_at))
        }

        if let Some(updated_at) = updated_at {
            filters.push(folder::updated_at::equals(*updated_at))
        }

        Ok(query)
    }
}
