use crate::{
    error::Error,
    folder::model::select::{folder_select, FolderSelect},
    prisma::{folder, Visibility},
};

use super::FolderService;

impl FolderService {
    pub async fn update_folder(
        &self,
        folder_id: String,
        parent: Option<String>,
        folder_name: Option<String>,
        visibility: Option<Visibility>,
    ) -> Result<FolderSelect, Error> {
        let mut changes = vec![];

        if let Some(parent) = parent {
            changes.push(folder::parent_folder_id::set(Some(parent)))
        }

        if let Some(folder_name) = folder_name.clone() {
            changes.push(folder::folder_name::set(folder_name))
        }

        if let Some(visibility) = visibility {
            changes.push(folder::visibility::set(visibility))
        }

        let updated_folder = self
            .db
            .folder()
            .update_unchecked(folder::id::equals(folder_id), changes)
            .select(folder_select::select())
            .exec()
            .await?;
        Ok(updated_folder)
    }
}
