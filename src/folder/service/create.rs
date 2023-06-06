use crate::{
    error::Error,
    folder::response::folder_response,
    prisma::{folder, Visibility},
};

use super::FolderService;

impl FolderService {
    pub async fn create_folder(
        &self,
        owner_id: String,
        folder_name: String,
        visibility: Option<Visibility>,
        parent_folder_id: String,
    ) -> Result<folder_response::Data, Error> {
        let new_folder = self
            .db
            .folder()
            .create_unchecked(
                owner_id,
                folder_name,
                visibility.unwrap_or(Visibility::Public),
                vec![folder::parent_folder_id::set(Some(parent_folder_id))],
            )
            .select(folder_response::select())
            .exec()
            .await?;
        Ok(new_folder)
    }
}
