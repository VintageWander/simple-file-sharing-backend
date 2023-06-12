use crate::{
    error::Error,
    file::model::select::{child_files_select, file_select, ChildFilesSelect, FileSelect},
    prisma::{
        file::{self, WhereParam},
        folder, user, Visibility,
    },
};

use super::FileService;

impl FileService {
    pub async fn get_child_files_from_folders(
        &self,
        parent_folder_id: Option<String>,
        child_files_filter: Vec<WhereParam>,
    ) -> Result<Vec<ChildFilesSelect>, Error> {
        let starting_point = match parent_folder_id {
            Some(parent_id) => folder::id::equals(parent_id),
            None => folder::parent_folder_id::equals(None),
        };

        let child_files = self
            .db
            .folder()
            .find_many(vec![starting_point])
            .select(child_files_select::select(child_files_filter))
            .exec()
            .await?
            .into_iter()
            .flat_map(|parent_folder| parent_folder.child_files)
            .collect();

        Ok(child_files)
    }

    pub async fn get_files_shared_to_user_id(
        &self,
        user_id: String,
        mut filters: Vec<WhereParam>,
    ) -> Result<Vec<FileSelect>, Error> {
        filters.extend(vec![
            file::visibility::equals(Visibility::Shared),
            file::collaborators::some(vec![user::id::equals(user_id)]),
        ]);

        let shared_files = self
            .db
            .file()
            .find_many(filters)
            .select(file_select::select())
            .exec()
            .await?;
        Ok(shared_files)
    }
}
