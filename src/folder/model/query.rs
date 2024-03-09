use axum::{
	async_trait,
	extract::{FromRequestParts, Query},
	http::request::Parts,
};
use chrono::{DateTime, FixedOffset};
use is_empty::IsEmpty;
use serde::Deserialize;
use validator::Validate;

use crate::{error::Error, prisma::Visibility, validation::uuid::check_uuid, GlobalState};

use super::validation::check_folder_name;

#[derive(Deserialize, Validate, IsEmpty)]
#[serde(rename_all = "camelCase")]
pub struct FolderQuery {
	#[validate(custom = "check_uuid")]
	pub id: Option<String>,

	#[validate(custom = "check_uuid")]
	pub owner_id: Option<String>,

	#[validate(custom = "check_uuid")]
	pub parent_folder_id: Option<String>,

	#[validate(custom = "check_folder_name")]
	pub folder_name: Option<String>,

	pub visibility: Option<Visibility>,

	pub created_at: Option<DateTime<FixedOffset>>,

	pub updated_at: Option<DateTime<FixedOffset>>,
}

#[async_trait]
impl FromRequestParts<GlobalState> for FolderQuery {
	type Rejection = Error;
	async fn from_request_parts(
		parts: &mut Parts,
		state: &GlobalState,
	) -> Result<Self, Self::Rejection> {
		let Query(query) = Query::<FolderQuery>::from_request_parts(parts, state).await?;

		Ok(query)
	}
}
