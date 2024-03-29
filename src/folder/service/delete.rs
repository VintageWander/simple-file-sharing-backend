use crate::{error::Error, folder::model::select::Folder, prisma::folder};

use super::FolderService;

impl FolderService {
	pub async fn delete_folder(&self, folder_id: String) -> Result<Folder, Error> {
		let deleted_folder = self
			.db
			.folder()
			.delete(folder::id::equals(folder_id))
			.exec()
			.await?;
		Ok(deleted_folder)
	}

	pub async fn delete_root_folder(&self, owner_id: String) -> Result<(), Error> {
		let deleted_folder = self.get_root_folder(owner_id).await?;

		self.delete_folder(deleted_folder).await?;

		Ok(())
	}
}
