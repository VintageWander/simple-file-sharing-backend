use crate::{
    error::Error,
    folder::response::{child_folders_response, folder_response},
    prisma::{
        folder::{self, WhereParam},
        user, Visibility,
    },
};

use super::FolderService;

impl FolderService {
    // This function returns child folders from a list of folders
    // param folders_filter to find which folders should be the parent
    // param child_folder_filter to filter the child folders from the folders_filter
    pub async fn get_child_folders_from_folders(
        &self,
        folders_filter: WhereParam, // Filter for the list of folders (parent level)
        child_folders_filter: Vec<WhereParam>, // Filter for the child folders of the above (child level)
    ) -> Result<Vec<child_folders_response::child_folders::Data>, Error> {
        let child_folders = self
            .db
            .folder()
            .find_many(vec![folders_filter])
            .select(child_folders_response::select(child_folders_filter))
            .exec()
            .await?
            .into_iter()
            .flat_map(|parent_folder| parent_folder.child_folders)
            .collect();
        Ok(child_folders)
    }

    pub async fn get_folders_shared_to_user_id(
        &self,
        user_id: String,
        mut filters: Vec<WhereParam>,
    ) -> Result<Vec<folder_response::Data>, Error> {
        filters.extend(vec![
            folder::visibility::equals(Visibility::Shared),
            folder::collaborators::some(vec![user::id::equals(user_id)]),
        ]);

        let shared_folders = self
            .db
            .folder()
            .find_many(filters)
            .select(folder_response::select())
            .exec()
            .await?;
        Ok(shared_folders)
    }
}
