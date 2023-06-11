use axum::http::{
    header::{
        ACCEPT, ACCESS_CONTROL_ALLOW_HEADERS, ACCESS_CONTROL_ALLOW_METHODS,
        ACCESS_CONTROL_ALLOW_ORIGIN, CONTENT_TYPE, ORIGIN,
    },
    HeaderValue, Method,
};
use const_env::from_env;
use tower_http::cors::CorsLayer;
use validator::{validate_url, ValidationError};

use crate::validation::check_with;

#[from_env]
pub const DATABASE_URL: &str = "";

#[from_env]
pub const PORT: u16 = 8000;

#[from_env("ORIGIN")]
pub const ENV_ORIGIN: &str = "";

#[from_env]
pub const ACCESS_TOKEN_SECRET: &str = "";

#[from_env]
pub const REFRESH_TOKEN_SECRET: &str = "";

#[from_env]
pub const ACCESS_KEY_ID: &str = "";

#[from_env]
pub const SECRET_ACCESS_KEY: &str = "";

#[from_env]
pub const BUCKET_NAME: &str = "";

#[from_env]
pub const REGION: &str = "";

pub fn check_env() {
    if !validate_url(ENV_ORIGIN) {
        panic!("Invalid origin");
    }
    if check_region(REGION).is_err() {
        panic!("Invalid region");
    }
    if check_db_connection(DATABASE_URL).is_err() {
        panic!("Invalid database connection");
    }
}

fn check_region(region: &str) -> Result<(), ValidationError> {
    check_with(
        region,
        r#"^([a-z]{2}-(central|(north|south)?(east|west)?)-\d)|((ap|ca|cn|eu|sa|us)-(central|(north|south)?(east|west)?)-\d)|((me|af|ap|eu|sa)-(south|north)?(east|west)?-\d)|((us-gov)-(east|west)-\d)$"#,
        "Not a valid AWS Region",
    )
}

fn check_db_connection(connection_string: &str) -> Result<(), ValidationError> {
    check_with(
        connection_string,
        r#"^postgres:\/\/[a-zA-Z0-9]+(:[a-zA-Z0-9]+)?@[a-zA-Z0-9]+(:[0-9]+)?\/[a-zA-Z0-9]+$"#,
        "The database connection string is incorrect",
    )
}

pub fn setup_cors() -> CorsLayer {
    CorsLayer::new()
        .allow_credentials(true)
        .allow_origin(
            ENV_ORIGIN
                .parse::<HeaderValue>()
                .expect("Failed to parse origin as HeaderValue"),
        )
        .allow_headers([
            ORIGIN,
            CONTENT_TYPE,
            ACCEPT,
            ACCESS_CONTROL_ALLOW_ORIGIN,
            ACCESS_CONTROL_ALLOW_METHODS,
            ACCESS_CONTROL_ALLOW_HEADERS,
        ])
        .allow_methods([
            Method::GET,
            Method::POST,
            Method::PUT,
            Method::DELETE,
            Method::OPTIONS,
        ])
}
