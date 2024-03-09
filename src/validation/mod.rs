use fancy_regex::Regex;
use validator::ValidationError;

pub mod uuid;

pub fn validation_message(msg: &'static str) -> ValidationError {
	let mut error = ValidationError::new("");
	error.message = Some(std::borrow::Cow::Borrowed(msg));
	error
}

pub fn check_with(
	test_str: &str,
	regex_str: &str,
	fail_message: &'static str,
) -> Result<(), ValidationError> {
	let regex = Regex::new(regex_str).map_err(|_| validation_message("Invalid Regex"))?;
	let result = regex
		.is_match(test_str)
		.map_err(|_| validation_message("Matching process failed"))?;

	match result {
		true => Ok(()),
		false => Err(validation_message(fail_message)),
	}
}
