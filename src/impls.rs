use axum::{async_trait, extract::multipart::Field};
use axum_typed_multipart::{TryFromField, TypedMultipartError};

use crate::prisma::Visibility;

#[async_trait]
impl TryFromField for Visibility {
    async fn try_from_field(field: Field<'_>) -> Result<Self, TypedMultipartError> {
        let visibility = match field.text().await?.as_str() {
            "public" => Visibility::Public,
            "private" => Visibility::Private,
            "shared" => Visibility::Shared,
            _ => {
                return Err(TypedMultipartError::WrongFieldType {
                    field_name: "visibility".into(),
                    wanted_type: "public, private, or shared".into(),
                })
            }
        };
        Ok(visibility)
    }
}
