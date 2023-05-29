use super::S3;
use crate::{
    error::Error,
    validation::file::{check_dir, check_fullpath},
};

impl S3 {
    pub async fn copy_file(&self, fullpath: &str, dest_fullpath: &str) -> Result<(), Error> {
        check_fullpath(fullpath)?;
        check_fullpath(dest_fullpath)?;

        let src = format!("{}/{fullpath}", self.bucket_name);

        self.client
            .copy_object()
            .copy_source(src)
            .bucket(&self.bucket_name)
            .key(dest_fullpath)
            .send()
            .await?;

        Ok(())
    }

    pub async fn copy_folder(&self, dir: &str, dest_dir: &str) -> Result<(), Error> {
        check_dir(dir)?;
        check_dir(dest_dir)?;

        let objs = self.get_all(dir).await?;

        for obj in objs {
            let src = format!("{}/{obj}", self.bucket_name);
            let dest = format!("{dest_dir}{}", obj.split_at(dir.len()).1);
            self.client
                .copy_object()
                .copy_source(src)
                .bucket(&self.bucket_name)
                .key(dest)
                .send()
                .await?;
        }
        Ok(())
    }
}
