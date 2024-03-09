use validator::ValidationError;

use crate::validation::check_with;

pub fn check_folder_name(folder_name: &str) -> Result<(), ValidationError> {
	check_with(
		folder_name,
		r#"^[a-zA-Z0-9-_]{3,20}$"#,
		"The folder name can only contain a-z A-Z 0-9 and within 3 to 20 characters in length",
	)
}

pub fn check_folder_name_option(folder_name: &Option<String>) -> Result<(), ValidationError> {
	if let Some(folder_name) = folder_name {
		check_with(
			folder_name,
			r#"^[a-zA-Z0-9-_]{3,20}$"#,
			"The folder name can only contain a-z A-Z 0-9 and within 3 to 20 characters in length",
		)
	} else {
		Ok(())
	}
}
