use axum::{
    extract::rejection::{JsonRejection, PathRejection},
    response::IntoResponse,
};
use prisma_client_rust::QueryError;
use thiserror::Error;
use validator::{ValidationError, ValidationErrors};

use crate::web::Web;

pub fn extract_validation_error(e: &ValidationErrors) -> String {
    let mut message = "".to_string();
    message.insert(0, 'F');
    for field in e.field_errors() {
        message.insert_str(message.len(), format!("ield {}: ", field.0).as_str());
        for field_error in field.1 {
            if let Some(msg) = &field_error.message {
                message.insert_str(message.len(), format!("{}; F", msg).as_str())
            } else {
                message.insert_str(message.len(), format!("{}; F", field_error.code).as_str())
            }
        }
    }
    message[0..message.len() - 3].to_string()
}

#[derive(Debug, Error)]
pub enum Error {
    /*
        Database errors
    */
    #[error("Query error")]
    Query(#[from] QueryError),

    /*
        Request parsing errors
    */
    #[error("Path parsing error")]
    Path(#[from] PathRejection),

    #[error("Json parse error")]
    Json(#[from] JsonRejection),

    /*
        Validation errors
    */
    #[error("Single invalid field")]
    SingleInvalidField(#[from] ValidationError),

    #[error("Multiple invalid fields")]
    MultipleInvalidFields(#[from] ValidationErrors),

    /*
        Authorization errors
    */
    #[error("JWT error")]
    Jwt(#[from] jsonwebtoken::errors::Error),

    #[error("Missing refresh token")]
    MissingRefreshToken,

    #[error("Unauthorized")]
    Unauthorized,

    #[error("Forbidden")]
    Forbidden,

    #[error("Decode token failed")]
    Decode,

    /*
        General errors
    */
    #[error("Not Found")]
    NotFound,
}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        match self {
            /*
                Database errors
            */
            Error::Query(e) => Web::bad_request(
                "Query error",
                format!(
                    "The information provided could not be found in the database. Error: {}",
                    e
                ),
            ),

            /*
                Request parsing errors
            */
            Error::Path(e) => Web::bad_request(
                "Path error",
                format!(
                    "The value in the path parameter cannot be used. Error: {}",
                    e
                ),
            ),
            Error::Json(e) => Web::bad_request("Json request error", e),

            /*
                Validation errors
            */
            Error::SingleInvalidField(e) => {
                Web::bad_request("One of the request fields might be incorrect", e)
            }
            Error::MultipleInvalidFields(e) => Web::bad_request(
                "Multiple request fields are invalid",
                extract_validation_error(&e),
            ),

            /*
                Authorization
            */
            Error::Jwt(e) => Web::bad_request(
                "Token error",
                format!("The token is invalid, cannot use. Error: {}", e),
            ),
            Error::MissingRefreshToken => {
                Web::bad_request("Refresh token not found", "Please provide a refresh token")
            }
            Error::Unauthorized => Web::unauthorized("Unauthorized", "You have to be logged in"),
            Error::Forbidden => {
                Web::forbidden("Forbidden", "You cannot perform actions the root folder")
            }
            Error::Decode => Web::bad_request(
                "Decode token failed",
                "This is due to your refresh token expired",
            ),

            /*
                General errors
            */
            Error::NotFound => Web::not_found(
                "Not found",
                "The value provided for query could not be found",
            ),
        }
    }
}
