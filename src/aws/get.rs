use aws_sdk_s3::primitives::ByteStream;

use super::S3;
use crate::{error::Error, file::model::validation::check_fullpath};

impl S3 {
    pub async fn get_all(&self, prefix: &str) -> Result<Vec<String>, Error> {
        let req = self
            .client
            .list_objects_v2()
            .bucket(&self.bucket_name)
            .prefix(prefix);
        let res = req.send().await?;
        let contents = res.contents().unwrap_or_default();
        let contents = contents
            .iter()
            .filter_map(|o| o.key.as_ref())
            .map(|s| s.to_string())
            .collect::<Vec<_>>();
        Ok(contents)
    }

    pub async fn get_data_by_key(&self, fullpath: &str) -> Result<ByteStream, Error> {
        check_fullpath(fullpath)?;
        let req = self
            .client
            .get_object()
            .bucket(&self.bucket_name)
            .key(fullpath);
        let res = req.send().await?;
        Ok(res.body)
    }
}
