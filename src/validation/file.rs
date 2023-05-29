use validator::ValidationError;

use super::check_with;

pub fn check_filename(filename: &str) -> Result<(), ValidationError> {
    // hello
    // This regex matches hello,
    // Rejects hello.txt and .txt
    // This can also match hello_world or hello-world
    check_with(
        filename,
        r#"^[a-zA-Z0-9-_]{3,20}$"#,
        "The name can only contain a-z A-Z 0-9 and within 3 to 20 characters in length",
    )
}

pub fn check_folder_name(folder_name: &str) -> Result<(), ValidationError> {
    check_filename(folder_name)
}

pub fn check_dir(position: &str) -> Result<(), ValidationError> {
    // This regex will match for cases like user/, user/hello/, hello-world/user/, hello_world/something/
    // It will reject cases like user, /user, /user/, or user/hello
    // Basically it requires a slash must exists at the end
    check_with(
        position,
        r#"^([a-zA-Z0-9-_]{3,50}[/])*$"#,
        "The dir input is in wrong format",
    )
}

pub fn check_fullpath(fullpath: &str) -> Result<(), ValidationError> {
    // This regex will match cases like user/hello.txt, hello.txt, or nested/something-deep/hello.txt
    // This will reject cases like hello/.txt, hello/world, or even hello/
    check_with(
        fullpath,
        r#"^(([a-zA-Z0-9-_]{3,50}[/])*)[a-zA-Z0-9-_]{3,50}\.(png|txt|jpg|jpeg|mp3)$"#,
        "The fullpath is incorrect",
    )
}
