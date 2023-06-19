use axum::response::Response;
use prisma_client_rust::{
    prisma_errors::query_engine::{
        ConstraintViolation, RecordNotFound, TableDoesNotExist, UniqueKeyViolation,
    },
    QueryError,
};

use crate::web::Web;

pub fn match_query_error(error: QueryError) -> Response {
    if error.is_prisma_error::<UniqueKeyViolation>() {
        Web::conflict(
            "Conflict data",
            "The provided data is already exists, please try another",
        )
    } else if error.is_prisma_error::<ConstraintViolation>() {
        Web::bad_request(
            "Constraint violated",
            "A constraint in the database has been violated",
        )
    } else if error.is_prisma_error::<RecordNotFound>() {
        Web::not_found(
            "Not found data",
            "The information provided could not be found in the database",
        )
    } else if error.is_prisma_error::<TableDoesNotExist>() {
        Web::internal_error(
            "Table does not exists",
            "The database has not yet been initialized",
        )
    } else {
        Web::internal_error("Unknown error", error)
    }
}
