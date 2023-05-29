use crate::{
    error::Error,
    validation::file::{check_dir, check_fullpath},
};

use super::S3;

impl S3 {
    pub async fn move_file(&self, fullpath: &str, dest_fullpath: &str) -> Result<(), Error> {
        check_fullpath(fullpath)?;
        check_fullpath(dest_fullpath)?;

        self.copy_file(fullpath, dest_fullpath).await?;
        self.delete_file(fullpath).await?;

        Ok(())
    }

    pub async fn move_folder(&self, dir_path: &str, dest_dir_path: &str) -> Result<(), Error> {
        check_dir(dir_path)?;
        check_dir(dest_dir_path)?;

        self.copy_folder(dir_path, dest_dir_path).await?;
        self.delete_folder(dir_path).await?;

        Ok(())
    }
}
