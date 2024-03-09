use crate::{
	config::{aws_bucket_name, aws_region, endpoint, minio},
	error::Error,
	file::model::validation::check_fullpath,
};

use bytes::Bytes;
use s3::{bucket::Bucket, creds::Credentials, request::ResponseDataStream, Region};
use tokio::fs::File;

#[derive(Debug, Clone)]
pub struct Storage {
	storage: Bucket,
}

impl Storage {
	pub fn init() -> Self {
		let bucket_name = aws_bucket_name();
		let region = aws_region();
		let region = match minio() {
			true => Region::Custom {
				region,
				endpoint: endpoint(),
			},
			false => region.parse().expect("Invalid REGION variable"),
		};
		let credentials = Credentials::from_env().expect("Cannot read AWS credentials from env");
		let bucket = Bucket::new(&bucket_name, region, credentials)
			.expect("Cannot get AWS S3 bucket")
			.with_path_style();

		Self { storage: bucket }
	}

	// Use this function to get a list of bytes
	pub async fn get_object(&self, full_filename: impl AsRef<str>) -> Result<Bytes, Error> {
		let res = self.storage.get_object(full_filename).await?;
		Ok(res.bytes().clone())
	}

	// Use this function in case you want to get the object and write it to a file
	pub async fn write_to_file(
		&self,
		full_filename: impl AsRef<str>,
		file: &mut File,
	) -> Result<ResponseDataStream, Error> {
		self.storage
			.get_object_stream(full_filename)
			.await
			.map_err(Into::into)
	}

	// Upload file
	pub async fn put_object(
		&self,
		full_filename: impl AsRef<str>,
		bytes: Bytes,
	) -> Result<(), Error> {
		let res = self.storage.put_object(full_filename, &bytes).await?;
		println!("{}", res.status_code());

		Ok(())
	}

	// Create new folder
	pub async fn put_folder(&self, folder_name: impl AsRef<str>) -> Result<(), Error> {
		self.storage
			.put_object(format!("{}/", folder_name.as_ref()), b"")
			.await?;

		Ok(())
	}

	// Move
	pub async fn move_object(&self, from: &str, to: &str) -> Result<(), Error> {
		check_fullpath(from)?;
		check_fullpath(to)?;

		// It is basically a rename,
		// but S3 does not have a rename function

		// Our only solution is to copy it to a new place, and delete the old one

		// Put it to the new path
		self.storage.copy_object_internal(from, to).await?;

		// Delete the old object
		self.delete_object(from).await?;

		Ok(())
	}

	// Delete
	pub async fn delete_object(&self, full_filename: impl AsRef<str>) -> Result<(), Error> {
		self.storage.delete_object(full_filename).await?;
		Ok(())
	}

	pub async fn delete_folder(&self, folder_name: impl AsRef<str>) -> Result<(), Error> {
		self.storage
			.delete_object(format!("{}/", folder_name.as_ref()))
			.await?;
		Ok(())
	}
}
