use crate::{
    error::Error,
    folder::response::folder_response,
    prisma::folder::{self, UncheckedSetParam},
};

use super::FolderService;

impl FolderService {
    pub async fn update_folder(
        &self,
        folder_id: String,
        changes: Vec<UncheckedSetParam>,
    ) -> Result<folder_response::Data, Error> {
        let updated_folder = self
            .db
            .folder()
            .update_unchecked(folder::id::equals(folder_id), changes)
            .select(folder_response::select())
            .exec()
            .await?;
        Ok(updated_folder)
    }
}
