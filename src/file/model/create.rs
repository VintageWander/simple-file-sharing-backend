use axum::{async_trait, body::Body, extract::FromRequest, http::Request};
use axum_typed_multipart::{FieldData, TryFromMultipart, TypedMultipart};
use bytes::Bytes;
use validator::Validate;

use crate::{error::Error, prisma::Visibility, validation::uuid::check_uuid_option, GlobalState};

#[derive(TryFromMultipart, Validate)]
pub struct CreateFileRequest {
	#[validate(custom(function = "check_uuid_option"))]
	pub parent: Option<String>,

	pub visibility: Option<Visibility>,

	pub file: FieldData<Bytes>,
}

#[async_trait]
impl FromRequest<GlobalState, Body> for CreateFileRequest {
	type Rejection = Error;
	async fn from_request(
		req: Request<Body>,
		state: &GlobalState,
	) -> Result<Self, Self::Rejection> {
		let TypedMultipart(body) =
			TypedMultipart::<CreateFileRequest>::from_request(req, state).await?;
		body.validate()?;
		Ok(body)
	}
}
