pub mod multipart;
pub mod print;
pub mod query;

use aws_sdk_s3::{
    error::SdkError,
    operation::{
        copy_object::CopyObjectError, delete_object::DeleteObjectError,
        delete_objects::DeleteObjectsError, get_object::GetObjectError,
        list_objects_v2::ListObjectsV2Error, put_object::PutObjectError,
    },
};
use axum::{
    extract::rejection::{JsonRejection, PathRejection, QueryRejection},
    response::IntoResponse,
};
use axum_typed_multipart::TypedMultipartError;
use prisma_client_rust::QueryError;

use thiserror::Error;
use validator::{ValidationError, ValidationErrors};

use crate::web::Web;

use self::{
    multipart::match_multipart_error, print::extract_validation_error, query::match_query_error,
};

#[derive(Debug, Error)]
pub enum Error {
    /*
        Database errors
    */
    #[error("Query error")]
    DatabaseQuery(#[from] QueryError),

    /*
        Request parsing errors
    */
    #[error("Path parsing error")]
    Path(#[from] PathRejection),

    #[error("Json parse error")]
    Json(#[from] JsonRejection),

    #[error("Query string parse error")]
    Query(#[from] QueryRejection),

    #[error("Multipart error")]
    Multipart(#[from] TypedMultipartError),

    /*
        Validation errors
    */
    #[error("Single invalid field")]
    SingleInvalidField(#[from] ValidationError),

    #[error("Multiple invalid fields")]
    MultipleInvalidFields(#[from] ValidationErrors),

    /*
        Authentication errors
    */
    #[error("JWT error")]
    Jwt(#[from] jsonwebtoken::errors::Error),

    #[error("Password hashing error")]
    PasswordHashing(#[from] argon2::password_hash::Error),

    #[error("Missing refresh token")]
    MissingRefreshToken,

    #[error("Unauthorized")]
    Unauthorized,

    #[error("Forbidden")]
    Forbidden,

    #[error("Decode token failed")]
    Decode,

    /*
        AWS errors
    */
    #[error("Upload file error")]
    PutObject(#[from] SdkError<PutObjectError>),

    #[error("Get single file error")]
    GetObject(#[from] SdkError<GetObjectError>),

    #[error("List all files error")]
    ListObject(#[from] SdkError<ListObjectsV2Error>),

    #[error("Copy file error")]
    CopyObject(#[from] SdkError<CopyObjectError>),

    #[error("Delete file error")]
    DeleteObject(#[from] SdkError<DeleteObjectError>),

    #[error("Delete files error")]
    DeleteObjects(#[from] SdkError<DeleteObjectsError>),

    /*
        General errors
    */
    #[error("Not Found")]
    NotFound,

    #[error("No content for response")]
    NoContent,

    /*
        Download error
    */
    #[error("Download error")]
    Download(#[from] std::io::Error),

    #[error("Zip error")]
    Zip(#[from] zip::result::ZipError),
}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        match self {
            /*
                Database errors
            */
            Error::DatabaseQuery(e) => match_query_error(e),

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
            Error::Query(e) => Web::bad_request("Query string invalid", e),
            Error::Multipart(e) => match_multipart_error(e),

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
                Authentication errors
            */
            Error::Jwt(e) => {
                Web::bad_request("Token error", format!("Please login again. Error: {}", e))
            }
            Error::PasswordHashing(e) => {
                Web::internal_error("Cannot hash the password", e)
            }
            Error::MissingRefreshToken => {
                Web::bad_request("Refresh token not found", "Please provide a refresh token")
            }
            Error::Unauthorized => Web::unauthorized("Unauthorized", "You have to be logged in"),
            Error::Forbidden => Web::forbidden(
                "Forbidden",
                "You do not have permissions to perform this action",
            ),
            Error::Decode => Web::bad_request(
                "Decode token failed",
                "This is due to your refresh token expired",
            ),

            /*
                AWS S3 errors
            */
            Error::GetObject(_) => Web::bad_request(
                "Get file error",
                "This maybe due to the information provided was incorrect",
            ),

            Error::PutObject(_) => Web::bad_request(
                "Cannot upload file",
                "This is maybe due to the file is in wrong format or too large for upload",
            ),

            Error::CopyObject(_) => Web::internal_error(
                "Cannot copy file",
                "This is maybe due to the storage server unable to copy, please try again later",
            ),

            Error::ListObject(_) => Web::internal_error(
                "Cannot get all files",
                "Probably database error, please try again later",
            ),

            Error::DeleteObject(_) => Web::internal_error(
                "Cannot delete file", 
                "This is due to the database have problems that prevent the file from being deleted"
            ),
            Error::DeleteObjects(_) => Web::internal_error(
                "Cannot delete multiple files",
                "This is due to database error, which make some files couldn't be deleted"
            ),


            /*
                General errors
            */
            Error::NotFound => Web::not_found(
                "Not found",
                "The value provided for query could not be found",
            ),
            Error::NoContent => Web::no_content(
                "None updated",
                "Since no data was provided, nothing has changed",
            ),

            /*
                Download errors
            */
            Error::Download(e) => Web::internal_error("The download process failed", e),
            Error::Zip(e) => Web::internal_error("The zip process failed", e)
        }
    }
}
