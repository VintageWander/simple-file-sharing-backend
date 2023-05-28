use std::borrow::Cow;

use axum::response::Response;
use prisma_client_rust::{prisma_errors::KnownError, QueryError};

use crate::web::Web;

pub fn match_query_error(error: QueryError) -> Response {
    match error {
        QueryError::Execute(e) => {
            let Some(KnownError{ error_code, message, ..}) = e.as_known() else {
                return Web::internal_error("Unknown error thrown", "Something gone wrong");
            };
            match error_code {
                Cow::Borrowed("P2002") => Web::conflict(
                    "Conflict data",
                    "The provided data is already exists, please try another",
                ),
                Cow::Borrowed("P2004") => Web::bad_request(
                    "Constraint violated",
                    "A constraint in the database has been violated",
                ),
                Cow::Borrowed("P2015") => Web::not_found(
                    "Not found data",
                    "The information provided could not be found in the database",
                ),
                _ => Web::internal_error("Unhandled error", format!("Error: {message}")),
            }
        }
        e => Web::bad_request("Serialize and Deserialize errors", e),
    }
}
