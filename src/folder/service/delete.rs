use crate::{error::Error, prisma::folder};

use super::FolderService;

impl FolderService {
    pub async fn delete_folder(&self, folder_id: String) -> Result<folder::Data, Error> {
        let deleted_folder = self
            .db
            .folder()
            .delete(folder::id::equals(folder_id))
            .exec()
            .await?;
        Ok(deleted_folder)
    }
}
