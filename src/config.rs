use axum::http::{
	header::{
		ACCEPT, ACCESS_CONTROL_ALLOW_HEADERS, ACCESS_CONTROL_ALLOW_METHODS,
		ACCESS_CONTROL_ALLOW_ORIGIN, CONTENT_TYPE, ORIGIN,
	},
	HeaderValue, Method,
};
use dotenvy::var;
use tower_http::cors::CorsLayer;
use validator::{validate_url, ValidationError};

use crate::validation::check_with;

pub fn port() -> u16 {
	var("PORT")
		.expect("PORT is not set")
		.parse()
		.expect("PORT is not a number")
}

pub fn hostname() -> String {
	var("HOSTNAME").unwrap_or("localhost".into())
}

pub fn https() -> bool {
	var("HTTPS").unwrap_or_default() == "true"
}

pub fn ssl_cert_key() -> (String, String) {
	if https() {
		(
			var("SSL_CERT").expect("Certificate path wasn't set"),
			var("SSL_KEY").expect("SSL key path wasn't set"),
		)
	} else {
		("".into(), "".into())
	}
}

pub fn origin() -> String {
	var("ORIGIN").expect("ORIGIN is not set")
}

pub fn access_token_secret() -> String {
	var("ACCESS_TOKEN_SECRET").expect("ACCESS_TOKEN_SECRET is not set")
}

pub fn refresh_token_secret() -> String {
	var("REFRESH_TOKEN_SECRET").expect("REFRESH_TOKEN_SECRET is not set")
}

pub fn share_key_secret() -> String {
	var("SHARE_KEY_SECRET").expect("SHARE_KEY_SECRET is not set")
}

pub fn aws_access_key_id() -> String {
	var("AWS_ACCESS_KEY_ID").expect("AWS_ACCESS_KEY_ID is not set")
}

pub fn aws_secret_access_key() -> String {
	var("AWS_SECRET_ACCESS_KEY").expect("AWS_SECRET_ACCESS_KEY is not set")
}

pub fn aws_bucket_name() -> String {
	var("AWS_BUCKET_NAME").expect("AWS_BUCKET_NAME is not set")
}

pub fn aws_region() -> String {
	var("AWS_REGION").expect("AWS_REGION is not set")
}

pub fn minio() -> bool {
	var("MINIO").unwrap_or_default() == "true"
}

pub fn endpoint() -> String {
	if minio() {
		var("ENDPOINT").expect("ENDPOINT is not set")
	} else {
		"".into()
	}
}

pub fn check_env() {
	if !validate_url(origin()) {
		panic!("Invalid origin");
	}
	if check_aws_region(&aws_region()).is_err() {
		panic!("Invalid aws_region");
	}
}

fn check_aws_region(aws_region: &str) -> Result<(), ValidationError> {
	check_with(
		aws_region,
		r"^([a-z]{2}-(central|(north|south)?(east|west)?)-\d)|((ap|ca|cn|eu|sa|us)-(central|(north|south)?(east|west)?)-\d)|((me|af|ap|eu|sa)-(south|north)?(east|west)?-\d)|((us-gov)-(east|west)-\d)$",
		"Not a valid AWS Region",
	)
}

pub fn setup_cors() -> CorsLayer {
	CorsLayer::new()
		.allow_credentials(true)
		.allow_origin(
			origin()
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
