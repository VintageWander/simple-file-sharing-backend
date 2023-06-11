use crate::{
    error::Error,
    folder::model::select::{folder_select, FolderSelect},
    prisma::folder::{self, UncheckedSetParam},
};

use super::FolderService;

impl FolderService {
    pub async fn update_folder(
        &self,
        folder_id: String,
        changes: Vec<UncheckedSetParam>,
    ) -> Result<FolderSelect, Error> {
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
