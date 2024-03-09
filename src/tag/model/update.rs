use axum::{async_trait, body::Body, extract::FromRequest, http::Request, Json};
use is_empty::IsEmpty;
use serde::Deserialize;
use validator::Validate;

use crate::{error::Error, GlobalState};

use super::validation::check_tag_name_option;

#[derive(Deserialize, IsEmpty, Validate)]
#[serde(rename_all = "camelCase")]
pub struct UpdateTagRequest {
	#[validate(custom(function = "check_tag_name_option"))]
	pub tag_name: Option<String>,
}

#[async_trait]
impl FromRequest<GlobalState, Body> for UpdateTagRequest {
	type Rejection = Error;
	async fn from_request(
		req: Request<Body>,
		state: &GlobalState,
	) -> Result<Self, Self::Rejection> {
		let Json(req) = Json::<UpdateTagRequest>::from_request(req, state).await?;

		if req.is_empty() {
			return Err(Error::NoContent);
		}

		Ok(req)
	}
}
