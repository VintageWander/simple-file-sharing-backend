use validator::ValidationError;

use crate::validation::check_with;

pub fn check_filename(filename: &str) -> Result<(), ValidationError> {
	/*
		hello
		This regex matches hello,
		Rejects hello.txt and .txt
		This can also match hello_world or hello-world
	*/
	check_with(
		filename,
		r#"^[a-zA-Z0-9-_]{3,50}$"#,
		"The name can only contain a-z A-Z 0-9 and within 3 to 50 characters in length",
	)
}

pub fn check_dir(position: &str) -> Result<(), ValidationError> {
	/*
		user/ || user/hello/ || hello-world/user/
		This regex will match for cases like user/, user/hello/, hello-world/user/, hello_world/something/
		It will reject cases like user, /user, /user/, or user/hello
		Basically it requires a slash must exists at the end
	*/
	check_with(
		position,
		r#"^([a-zA-Z0-9-_]{3,50}[/])*$"#,
		"The dir input is in wrong format",
	)
}

pub fn check_fullpath(fullpath: &str) -> Result<(), ValidationError> {
	/*
		user/hello.txt
		This regex will match cases like user/hello.txt, hello.txt, or nested/something-deep/hello.txt
		This will reject cases like hello/.txt, hello/world, or even hello/
	*/
	check_with(
		fullpath,
		r"^(([a-zA-Z0-9-_]{3,50}[/])*)[a-zA-Z0-9-_]{3,50}\.(png|txt|jpg|jpeg|mp3)$",
		"The fullpath is incorrect",
	)
}
