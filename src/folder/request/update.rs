use axum::{async_trait, body::Body, extract::FromRequest, http::Request, Json};
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
pub struct UpdateFolderRequest {
    #[validate(custom = "check_uuid")]
    pub parent: Option<String>,

    #[validate(custom = "check_folder_name")]
    pub folder_name: Option<String>,

    pub visibility: Option<Visibility>,

    #[serde(skip)]
    #[is_empty(if = "Vec::is_empty")]
    pub actions: Vec<folder::UncheckedSetParam>,
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
            parent,
            folder_name,
            visibility,
            ref mut actions,
        } = &mut body;

        if let Some(parent) = parent.clone() {
            actions.push(folder::parent_folder_id::set(Some(parent)))
        }

        if let Some(folder_name) = folder_name.clone() {
            actions.push(folder::folder_name::set(folder_name))
        }

        if let Some(visibility) = visibility {
            actions.push(folder::visibility::set(*visibility))
        }

        Ok(body)
    }
}
