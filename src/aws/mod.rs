pub mod copy;
pub mod create;
pub mod delete;
pub mod get;
pub mod rename;
pub mod transfer;

use aws_sdk_s3::{
    config::{self, Credentials, Region},
    Client,
};

use crate::config::{ACCESS_KEY_ID, BUCKET_NAME, REGION, SECRET_ACCESS_KEY};

#[derive(Debug, Clone)]
pub struct S3 {
    client: Client,
    bucket_name: String,
}

impl S3 {
    pub fn init() -> Self {
        // Build the aws cred
        let cred = Credentials::new(ACCESS_KEY_ID, SECRET_ACCESS_KEY, None, None, "get-from-env");

        // Build the aws config
        let region = Region::new(REGION);

        let conf_builder = config::Builder::new()
            .region(region)
            .credentials_provider(cred);

        let conf = conf_builder.build();

        Self {
            client: Client::from_conf(conf),
            bucket_name: BUCKET_NAME.to_string(),
        }
    }
}
