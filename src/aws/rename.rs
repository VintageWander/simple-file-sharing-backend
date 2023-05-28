use super::S3;
use crate::{
    error::Error,
    validation::{
        file::{check_dir, check_fullpath},
        validation_message,
    },
};

pub fn get_folder_position(str: &str) -> Result<String, Error> {
    check_dir(str)?;
    let mut result = str.split('/').collect::<Vec<_>>();
    result.pop();
    result.pop();
    let mut result = result.join("/");
    if result.is_empty() {
        return Ok(result);
    }
    result += "/";
    Ok(result)
}

pub fn get_file_position(str: &str) -> Result<String, Error> {
    check_fullpath(str)?;
    let mut result = str.split('/').collect::<Vec<_>>();
    result.pop();
    let mut result = result.join("/");
    result += "/";
    Ok(result)
}

impl S3 {
    pub async fn rename_file(&self, fullpath: &str, rename_path: &str) -> Result<(), Error> {
        check_fullpath(fullpath);
        check_fullpath(rename_path);

        if get_folder_position(fullpath)? != get_folder_position(rename_path)? {
            return Err(validation_message(
                "This isn't a rename, use the move file function if you want to move",
            )
            .into());
        }

        if fullpath == rename_path {
            return Ok(());
        }
        self.move_file(fullpath, rename_path).await
    }

    pub async fn rename_folder(&self, dir_path: &str, rename_path: &str) -> Result<(), Error> {
        check_dir(dir_path);
        check_dir(rename_path);

        if get_folder_position(dir_path)? != get_folder_position(rename_path)? {
            return Err(validation_message(
                "This isn't a rename, use the move folder function if you want to move",
            )
            .into());
        }

        if dir_path == rename_path {
            return Ok(());
        }
        self.move_folder(dir_path, rename_path).await
    }
}
