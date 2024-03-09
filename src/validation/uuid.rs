use std::collections::HashSet;

use validator::ValidationError;

use super::check_with;

pub fn check_uuid_option(uuid: &Option<String>) -> Result<(), ValidationError> {
	if let Some(uuid) = uuid {
		check_with(
			uuid,
			r"^[0-9a-fA-F]{8}\b-[0-9a-fA-F]{4}\b-[0-9a-fA-F]{4}\b-[0-9a-fA-F]{4}\b-[0-9a-fA-F]{12}$",
			"The id provided is not an UUID",
		)
	} else {
		Ok(())
	}
}

pub fn check_uuid(uuid: &str) -> Result<(), ValidationError> {
	check_with(
		uuid,
		r"^[0-9a-fA-F]{8}\b-[0-9a-fA-F]{4}\b-[0-9a-fA-F]{4}\b-[0-9a-fA-F]{4}\b-[0-9a-fA-F]{12}$",
		"The id provided is not an UUID",
	)
}

pub fn check_uuid_set(uuids: &HashSet<String>) -> Result<(), ValidationError> {
	for uuid in uuids {
		check_with(
			uuid,
			r"^[0-9a-fA-F]{8}\b-[0-9a-fA-F]{4}\b-[0-9a-fA-F]{4}\b-[0-9a-fA-F]{4}\b-[0-9a-fA-F]{12}$",
			"The id provided is not an UUID",
		)?;
	}
	Ok(())
}
