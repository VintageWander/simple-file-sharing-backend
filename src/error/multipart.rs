use axum::{http::StatusCode, response::Response};
use axum_typed_multipart::TypedMultipartError;
use serde_json::json;

use crate::web::Web;

pub fn match_multipart_error(e: TypedMultipartError) -> Response {
    let code = match e {
        TypedMultipartError::Other { .. } => StatusCode::INTERNAL_SERVER_ERROR,
        _ => StatusCode::BAD_REQUEST,
    };
    Web::new(code, "Multipart error", json!(null), e.to_string())
}
