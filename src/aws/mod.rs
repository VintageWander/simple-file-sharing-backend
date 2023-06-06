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

use crate::config::Config;

#[derive(Debug, Clone)]
pub struct S3 {
    client: Client,
    bucket_name: String,
}

impl S3 {
    pub fn init(config: &Config) -> Self {
        // Get the id secret from env
        let Config {
            s3_key_id,
            s3_key_secret,
            region,
            bucket_name,
            ..
        } = config;

        // Build the aws cred
        let cred = Credentials::new(s3_key_id, s3_key_secret, None, None, "get-from-env");

        // Build the aws config
        let region = Region::new(region.clone());

        let conf_builder = config::Builder::new()
            .region(region)
            .credentials_provider(cred);

        let conf = conf_builder.build();

        Self {
            client: Client::from_conf(conf),
            bucket_name: bucket_name.clone(),
        }
    }
}
