use axum::{
	http::StatusCode,
	response::{IntoResponse, Response},
	Json,
};
use http_serde::status_code;
use serde::Serialize;
use serde_json::{json, Value};

#[derive(Serialize)] 
pub struct Web {
	#[serde(with = "status_code")]
	pub code: StatusCode,
	pub message: String,
	pub data: Value,
	pub error: String,
}

#[allow(clippy::new_ret_no_self)]
impl Web {
	pub fn new(
		code: StatusCode,
		message: impl ToString,
		data: impl Serialize,
		error: impl ToString,
	) -> Response {
		(
			code,
			Json(Web {
				code,
				message: message.to_string(),
				data: json!(&data),
				error: error.to_string(),
			}),
		)
			.into_response()
	}

	pub fn ok(message: impl ToString, data: impl Serialize) -> Response {
		Web::new(StatusCode::OK, message, data, "")
	}

	pub fn created(message: impl ToString, data: impl Serialize) -> Response {
		Web::new(StatusCode::CREATED, message, data, "")
	}

	pub fn no_content(message: impl ToString, data: impl Serialize) -> Response {
		Web::new(StatusCode::NO_CONTENT, message, data, "")
	}

	pub fn unauthorized(message: impl ToString, error: impl ToString) -> Response {
		Web::new(StatusCode::UNAUTHORIZED, message, json!(null), error)
	}

	pub fn forbidden(message: impl ToString, error: impl ToString) -> Response {
		Web::new(StatusCode::NOT_FOUND, message, json!(null), error)
	}

	pub fn conflict(message: impl ToString, error: impl ToString) -> Response {
		Web::new(StatusCode::CONFLICT, message, json!(null), error)
	}

	pub fn bad_request(message: impl ToString, error: impl ToString) -> Response {
		Web::new(StatusCode::BAD_REQUEST, message, json!(null), error)
	}

	pub fn not_found(message: impl ToString, error: impl ToString) -> Response {
		Web::new(StatusCode::NOT_FOUND, message, json!(null), error)
	}

	pub fn internal_error(message: impl ToString, error: impl ToString) -> Response {
		Web::new(
			StatusCode::INTERNAL_SERVER_ERROR,
			message,
			json!(null),
			error,
		)
	}
}
