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
	validation::uuid::check_uuid_option,
	GlobalState,
};

use super::validation::check_filename_option;

#[derive(Deserialize, Validate, IsEmpty)]
#[serde(rename_all = "camelCase")]
pub struct FileQuery {
	#[validate(custom(function = "check_uuid_option"))]
	pub id: Option<String>,

	#[validate(custom(function = "check_uuid_option"))]
	pub owner_id: Option<String>, // ignored

	#[validate(custom(function = "check_uuid_option"))]
	pub parent_folder_id: Option<String>, // ignored

	#[validate(custom(function = "check_filename_option"))]
	pub filename: Option<String>,

	pub extension: Option<Extension>,

	pub visibility: Option<Visibility>, // ignored

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

		Ok(query)
	}
}
