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

use crate::config::{
    aws_access_key_id, aws_bucket_name, aws_region, aws_secret_access_key, endpoint, minio,
};

#[derive(Debug, Clone)]
pub struct S3 {
    client: Client,
    bucket_name: String,
}

impl S3 {
    pub fn init() -> Self {
        // Get the id secret from env

        // Build the aws cred
        let cred = Credentials::new(
            aws_access_key_id(),
            aws_secret_access_key(),
            None,
            None,
            "get-from-env",
        );

        // Build the aws config
        let region = Region::new(aws_region());

        let mut conf_builder = config::Builder::new()
            .region(region)
            .credentials_provider(cred);

        if minio() {
            conf_builder = conf_builder.endpoint_url(endpoint()).force_path_style(true)
        }

        let conf = conf_builder.build();

        Self {
            client: Client::from_conf(conf),
            bucket_name: aws_bucket_name(),
        }
    }
}
