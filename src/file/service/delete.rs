use crate::{error::Error, file::model::select::File, prisma::file};

use super::FileService;

impl FileService {
    pub async fn delete_file(&self, file_id: String) -> Result<File, Error> {
        let deleted_file = self
            .db
            .file()
            .delete(file::id::equals(file_id))
            .exec()
            .await?;
        Ok(deleted_file)
    }
}
