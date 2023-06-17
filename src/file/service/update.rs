use crate::{
    error::Error,
    file::model::select::{file_select, FileSelect},
    prisma::{file, Visibility},
};

use super::FileService;

impl FileService {
    pub async fn update_file(
        &self,
        file_id: String,
        parent: Option<String>,
        full_filename: Option<String>,
        visibility: Option<Visibility>,
    ) -> Result<FileSelect, Error> {
        let mut changes = vec![];

        if let Some(parent) = parent {
            changes.push(file::parent_folder_id::set(parent))
        }

        if let Some(full_filename) = full_filename {
            let (filename, _) = full_filename
                .split_once('.')
                .expect("This shouldn't be happening");
            changes.push(file::filename::set(filename.to_string()));
        }

        if let Some(visibility) = visibility {
            changes.push(file::visibility::set(visibility))
        }

        let updated_file = self
            .db
            .file()
            .update(file::id::equals(file_id), changes)
            .select(file_select::select())
            .exec()
            .await?;
        Ok(updated_file)
    }
}
