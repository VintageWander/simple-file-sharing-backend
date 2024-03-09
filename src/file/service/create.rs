use crate::{
	error::Error,
	file::model::select::{file_select, FileSelect},
	prisma::{folder, user, Extension, Visibility},
};

use super::FileService;

impl FileService {
	pub async fn create_file(
		&self,
		parent_folder_id: String,
		full_filename: String,
		visibility: Visibility,
		owner_id: String,
	) -> Result<FileSelect, Error> {
		let (filename, extension) = full_filename
			.split_once('.')
			.expect("This error should not be happening");
		let extension = Extension::try_from(extension)?;
		let new_file = self
			.db
			.file()
			.create(
				user::id::equals(owner_id),
				folder::id::equals(parent_folder_id),
				filename.to_string(),
				extension,
				visibility,
				vec![],
			)
			.select(file_select::select())
			.exec()
			.await?;
		Ok(new_file)
	}
}
