use std::sync::Arc;

use chrono::Utc;

use crate::{
	error::Error,
	prisma::{file, file_version, PrismaClient},
};

use super::model::{file_version_select, FileVersionSelect};

#[derive(Clone)]
pub struct FileVersionService {
	db: Arc<PrismaClient>,
}

impl FileVersionService {
	pub fn init(db: &Arc<PrismaClient>) -> Self {
		Self { db: db.clone() }
	}

	pub async fn get_versions_by_file_id(
		&self,
		file_id: String,
	) -> Result<Vec<FileVersionSelect>, Error> {
		let versions = self
			.db
			.file_version()
			.find_many(vec![file_version::file_id::equals(file_id)])
			.select(file_version_select::select())
			.exec()
			.await?;
		Ok(versions)
	}

	pub async fn get_version_by_file_id(
		&self,
		file_id: String,
		version_number: i64,
	) -> Result<FileVersionSelect, Error> {
		let version = self
			.db
			.file_version()
			.find_unique(file_version::file_id_version_number(
				file_id,
				version_number,
			))
			.select(file_version_select::select())
			.exec()
			.await?
			.ok_or_else(|| Error::NotFound)?;
		Ok(version)
	}

	pub async fn create_version_for_file(
		&self,
		file_id: String,
	) -> Result<FileVersionSelect, Error> {
		let new_version = self
			.db
			.file_version()
			.create(
				file::id::equals(file_id),
				Utc::now().timestamp_millis(),
				vec![],
			)
			.select(file_version_select::select())
			.exec()
			.await?;

		Ok(new_version)
	}

	pub async fn delete_version_from_file(
		&self,
		file_id: String,
		version_number: i64,
	) -> Result<FileVersionSelect, Error> {
		let deleted_version = self
			.db
			.file_version()
			.delete(file_version::file_id_version_number(
				file_id,
				version_number,
			))
			.select(file_version_select::select())
			.exec()
			.await?;
		Ok(deleted_version)
	}
}
