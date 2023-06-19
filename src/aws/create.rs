use aws_sdk_s3::primitives::SdkBody;
use bytes::Bytes;

use super::S3;
use crate::{
    error::Error,
    file::model::validation::{check_dir, check_fullpath},
};

impl S3 {
    pub async fn create_file(&self, fullpath: &str, data: Bytes) -> Result<(), Error> {
        check_fullpath(fullpath)?;

        let mime = mime_guess::from_path(fullpath)
            .first_or_octet_stream()
            .to_string();

        self.client
            .put_object()
            .bucket(&self.bucket_name)
            .key(fullpath)
            .body(data.into())
            .content_type(&mime)
            .send()
            .await?;

        Ok(())
    }

    pub async fn create_folder(&self, fullpath: &str) -> Result<(), Error> {
        check_dir(fullpath)?;

        self.client
            .put_object()
            .bucket(&self.bucket_name)
            .key(fullpath)
            .body(SdkBody::empty().into())
            .send()
            .await?;
        Ok(())
    }
}
