use validator::ValidationError;

use crate::validation::check_with;

pub fn check_username(username: &str) -> Result<(), ValidationError> {
    check_with(
        username,
        r#"^[a-zA-Z0-9-_]{2,}$"#,
        "Username must be a-z, A-Z, 0-9, -_ and at least 2 characters",
    )
}

pub fn check_password(password: &str) -> Result<(), ValidationError> {
    check_with(
        password, 
        r"^((?=.*[A-Z])(?=.*[a-z])(?=.*[0-9])(?=.*\W)).{8,}$",
         "Password must contains at least one lowercase, one uppercase, one digit, one special character, and at least 8 characters in length"
    )
}