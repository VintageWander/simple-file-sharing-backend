use validator::ValidationError;

use super::check_with;

pub fn check_filename(filename: &str) -> Result<(), ValidationError> {
    // hello
    // This regex matches hello,
    // Rejects hello.txt and .txt
    // This can also match hello_world or hello-world
    check_with(
        filename,
        r#"^[a-zA-Z0-9-_]{3,}$"#,
        "The name can only contain a-z A-Z 0-9 and within 3 to 20 characters in length",
    )
}

pub fn check_folder_name(folder_name: &str) -> Result<(), ValidationError> {
    check_filename(folder_name)
}
