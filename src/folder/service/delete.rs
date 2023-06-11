use crate::{
    error::Error,
    folder::model::select::Folder,
    prisma::{folder, Visibility},
};

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

    pub async fn delete_root_folder(&self, owner_id: String) -> Result<Folder, Error> {
        let deleted_folder = self
            .db
            .folder()
            .find_first(vec![
                folder::owner_id::equals(owner_id.clone()),
                folder::folder_name::equals(owner_id),
                folder::visibility::equals(Visibility::Private),
                folder::parent_folder_id::equals(None),
            ])
            .exec()
            .await?
            .ok_or_else(|| Error::NotFound)?;

        self.delete_folder(deleted_folder.id.clone()).await?;

        Ok(deleted_folder)
    }
}
