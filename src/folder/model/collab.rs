use std::collections::HashSet;

use axum::{async_trait, body::Body, extract::FromRequest, http::Request, Json};
use serde::Deserialize;
use validator::Validate;

use crate::{
	error::Error,
	validation::uuid::{check_uuid, check_uuid_set},
	GlobalState,
};

#[derive(Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct SetFolderCollabRequest {
	#[validate(custom(function = "check_uuid"))]
	pub folder_id: String,

	#[validate(custom(function = "check_uuid_set"))]
	pub user_ids: HashSet<String>,
}

#[async_trait]
impl FromRequest<GlobalState, Body> for SetFolderCollabRequest {
	type Rejection = Error;
	async fn from_request(
		req: Request<Body>,
		state: &GlobalState,
	) -> Result<Self, Self::Rejection> {
		let Json(req) = Json::<SetFolderCollabRequest>::from_request(req, state).await?;

		req.validate()?;

		Ok(req)
	}
}
