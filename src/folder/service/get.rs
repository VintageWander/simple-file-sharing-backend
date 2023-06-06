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
    /*
        This function returns child folders from a list of root folders (or one folder, based on the first param)

        param parent_folder_id to find specify which folder should be the parent
        If Some -> the id will be used to get that folder
        If None -> get all root folders

        param child_folder_filter to filter the child folders from the folders_filter
    */

    pub async fn get_child_folders_from_folders(
        &self,
        parent_folder_id: Option<String>, // Parent folder filter
        child_folders_filter: Vec<WhereParam>, // Filter for the child folders of the above (child level)
    ) -> Result<Vec<child_folders_response::child_folders::Data>, Error> {
        //
        let starting_point = match parent_folder_id {
            Some(parent_id) => folder::id::equals(parent_id),
            None => folder::parent_folder_id::equals(None),
        };

        let child_folders = self
            .db
            .folder()
            .find_many(vec![starting_point])
            .select(child_folders_response::select(child_folders_filter))
            .exec()
            .await?
            .into_iter()
            .flat_map(|parent_folder| parent_folder.child_folders)
            .collect();
        Ok(child_folders)
    }

    // Get all "Shared to me" folders, by user_id
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

    pub async fn get_folder_by_user_id(
        &self,
        folder_filter: Vec<WhereParam>,
        user_id: String,
    ) -> Result<folder_response::Data, Error> {
        /*
            There are 2 things that we have to deal with, that is
            We'll use the folder_filter to get the folder
        */
        todo!()
    }
}
