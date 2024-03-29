use validator::ValidationError;

use crate::validation::check_with;

pub fn check_tag_name(tag_name: &str) -> Result<(), ValidationError> {
	check_with(
		tag_name,
		r#"^[a-zA-Z0-9._-]{1,20}$"#,
		"The tag name is invalid",
	)
}

pub fn check_tag_name_option(tag_name: &Option<String>) -> Result<(), ValidationError> {
	if let Some(tag_name) = tag_name {
		check_with(
			tag_name,
			r#"^[a-zA-Z0-9._-]{1,20}$"#,
			"The tag name is invalid",
		)
	} else {
		Ok(())
	}
}
