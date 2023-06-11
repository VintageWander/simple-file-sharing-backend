use crate::{
    error::Error,
    folder::model::select::{folder_select, FolderSelect},
    prisma::{folder, Visibility},
};

use super::FolderService;

impl FolderService {
    pub async fn create_folder(
        &self,
        owner_id: String,
        folder_name: String,
        visibility: Option<Visibility>,
        parent: Option<String>,
    ) -> Result<FolderSelect, Error> {
        let new_folder = self
            .db
            .folder()
            .create_unchecked(
                owner_id,
                folder_name,
                visibility.unwrap_or(Visibility::Public),
                vec![folder::parent_folder_id::set(parent)],
            )
            .select(folder_select::select())
            .exec()
            .await?;
        Ok(new_folder)
    }

    pub async fn create_root_folder(&self, owner_id: String) -> Result<FolderSelect, Error> {
        let root_folder = self
            .create_folder(owner_id.clone(), owner_id, Some(Visibility::Private), None)
            .await?;
        Ok(root_folder)
    }
}
