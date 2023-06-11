use axum::{async_trait, body::Body, extract::FromRequest, http::Request, Json};
use is_empty::IsEmpty;
use serde::Deserialize;
use validator::Validate;

use crate::{
    error::Error,
    prisma::{folder, Visibility},
    validation::uuid::check_uuid,
    GlobalState,
};

use super::validation::check_folder_name;

#[derive(Deserialize, Validate, IsEmpty)]
#[serde(rename_all = "camelCase")]
pub struct UpdateFolderRequest {
    #[validate(custom = "check_uuid")]
    pub parent: Option<String>,

    #[validate(custom = "check_folder_name")]
    pub folder_name: Option<String>,

    pub visibility: Option<Visibility>,

    #[serde(skip)]
    #[is_empty(if = "Vec::is_empty")]
    pub changes: Vec<folder::UncheckedSetParam>,
}

#[async_trait]
impl FromRequest<GlobalState, Body> for UpdateFolderRequest {
    type Rejection = Error;
    async fn from_request(
        req: Request<Body>,
        state: &GlobalState,
    ) -> Result<Self, Self::Rejection> {
        let Json(mut body) = Json::<UpdateFolderRequest>::from_request(req, state).await?;

        if body.is_empty() {
            return Err(Error::NoContent);
        }

        body.validate()?;

        let UpdateFolderRequest {
            folder_name,
            visibility,
            ref mut changes,
            ..
        } = &mut body;

        if let Some(folder_name) = folder_name.clone() {
            changes.push(folder::folder_name::set(folder_name))
        }

        if let Some(visibility) = visibility {
            changes.push(folder::visibility::set(*visibility))
        }

        Ok(body)
    }
}
