use axum::http::{
    header::{
        ACCEPT, ACCESS_CONTROL_ALLOW_HEADERS, ACCESS_CONTROL_ALLOW_METHODS,
        ACCESS_CONTROL_ALLOW_ORIGIN, CONTENT_TYPE, ORIGIN,
    },
    HeaderValue, Method,
};
use envy::from_env;
use serde::Deserialize;
use tower_http::cors::CorsLayer;
use validator::Validate;

use crate::{error::print::extract_validation_error, validation::aws::check_region};

#[derive(Deserialize, Clone, Validate)]
pub struct Config {
    pub port: u16,

    #[validate(url(message = "ORIGIN invalid"))]
    pub origin: String,

    pub jwt_access: String,
    pub jwt_refresh: String,

    pub s3_key_id: String,
    pub s3_key_secret: String,
    pub bucket_name: String,
    #[validate(custom = "check_region")]
    pub region: String,
}

impl Config {
    pub fn from_env() -> Config {
        let config = match from_env::<Config>() {
            Ok(config) => config,
            Err(e) => panic!("{}", e.to_string()),
        };
        if let Err(e) = config.validate() {
            panic!("{}", extract_validation_error(&e))
        }
        config
    }

    pub fn setup_cors(origin: String) -> CorsLayer {
        CorsLayer::new()
            .allow_credentials(true)
            .allow_origin(
                origin
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
}
