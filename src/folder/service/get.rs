use std::collections::VecDeque;

use chrono::{DateTime, FixedOffset};

use crate::{
    error::Error,
    folder::model::select::{
        child_folders_select, folder_select, ChildFoldersSelect, FolderSelect,
    },
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

        The rest of the fields are used as filters for the child folders
    */
    #[allow(clippy::too_many_arguments)]
    pub async fn get_child_folders_from_folders(
        &self,
        id: Option<String>,
        owner_id: Option<String>,
        parent_folder_id: Option<String>,
        folder_name: Option<String>,
        visibility: Option<Visibility>,
        created_at: Option<DateTime<FixedOffset>>,
        updated_at: Option<DateTime<FixedOffset>>,
    ) -> Result<Vec<ChildFoldersSelect>, Error> {
        //
        let starting_point = match parent_folder_id {
            Some(parent_id) => folder::id::equals(parent_id),
            None => folder::parent_folder_id::equals(None),
        };

        let mut child_folders_filter = vec![];

        if let Some(id) = id {
            child_folders_filter.push(folder::id::equals(id))
        };

        if let Some(owner_id) = owner_id {
            child_folders_filter.push(folder::owner_id::equals(owner_id))
        };

        if let Some(folder_name) = folder_name {
            child_folders_filter.push(folder::folder_name::equals(folder_name))
        };

        if let Some(visibility) = visibility {
            child_folders_filter.push(folder::visibility::equals(visibility))
        };

        if let Some(created_at) = created_at {
            child_folders_filter.push(folder::created_at::equals(created_at))
        }

        if let Some(updated_at) = updated_at {
            child_folders_filter.push(folder::updated_at::equals(updated_at))
        }

        let child_folders = self
            .db
            .folder()
            .find_many(vec![starting_point])
            .select(child_folders_select::select(child_folders_filter))
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
        id: Option<String>,
        parent_folder_id: Option<String>,
        folder_name: Option<String>,
        created_at: Option<DateTime<FixedOffset>>,
        updated_at: Option<DateTime<FixedOffset>>,
    ) -> Result<Vec<FolderSelect>, Error> {
        let mut filters = vec![];

        if let Some(id) = id {
            filters.push(folder::id::equals(id))
        }

        if let Some(parent_folder_id) = parent_folder_id {
            filters.push(folder::parent_folder_id::equals(Some(parent_folder_id)))
        }

        if let Some(folder_name) = folder_name {
            filters.push(folder::folder_name::equals(folder_name))
        }

        if let Some(created_at) = created_at {
            filters.push(folder::created_at::equals(created_at))
        }

        if let Some(updated_at) = updated_at {
            filters.push(folder::updated_at::equals(updated_at))
        }

        filters.extend(vec![
            folder::visibility::equals(Visibility::Shared),
            folder::collaborators::some(vec![user::id::equals(user_id)]),
        ]);

        let shared_folders = self
            .db
            .folder()
            .find_many(filters)
            .select(folder_select::select())
            .exec()
            .await?;
        Ok(shared_folders)
    }

    pub async fn get_folder_by_user_id(
        &self,
        folder_filter: Vec<WhereParam>,
        user_id: String,
    ) -> Result<FolderSelect, Error> {
        /*
            There are 2 things that we have to deal with, that is
            We'll use the folder_filter to get the folder
            Then prepare 2 queries,
            1. Search the folder by owner_id
            2. Search the folder's collaborators list to see if there's one user_id equals to us

            We'll execute the first one, if it returns true, then return the folder
            If it returns false, try the second one, if that is true, then return the folder
            Finally, if both fails, we return a NotFound error

            The operation will cost at most 2 queries, and each query returns only one folder
        */

        let user_id_filter = folder_filter
            .clone()
            .into_iter()
            .chain(vec![folder::owner_id::equals(user_id.clone())])
            .collect();

        let collaborator_filter = folder_filter
            .into_iter()
            .chain(vec![
                folder::visibility::equals(Visibility::Shared),
                folder::collaborators::some(vec![user::id::equals(user_id)]),
            ])
            .collect();

        let search_by_user_id = self
            .db
            .folder()
            .find_first(user_id_filter)
            .select(folder_select::select())
            .exec();

        let search_from_collaborators = self
            .db
            .folder()
            .find_first(collaborator_filter)
            .select(folder_select::select())
            .exec();

        match search_by_user_id.await? {
            Some(owned_folder) => Ok(owned_folder),
            None => match search_from_collaborators.await? {
                Some(shared_to_user_folder) => Ok(shared_to_user_folder),
                None => Err(Error::NotFound),
            },
        }
    }

    /*
        What this function does is that it gets all files that lives under the folder_id
        Even child files, grandchild files, grandgrandchild files, all of them
    */
    pub async fn get_nested_files_from_folder(
        &self,
        folder_id: String,
    ) -> Result<VecDeque<String>, Error> {
        let mut folders_queue = VecDeque::new();
        let mut files_queue = VecDeque::new();

        folders_queue.push_back(folder_id);

        while !folders_queue.is_empty() {
            let first_folder = folders_queue[0].clone();

            let first_folder = self
                .db
                .folder()
                .find_unique(folder::id::equals(first_folder))
                .select(folder::select!({
                    child_files: select {
                        id
                    }
                    child_folders: select {
                        id
                    }
                }))
                .exec()
                .await?
                .ok_or_else(|| Error::NotFound)?;

            folders_queue.extend(first_folder.child_folders.into_iter().map(|f| f.id));
            files_queue.extend(first_folder.child_files.into_iter().map(|f| f.id));

            folders_queue.pop_front();
        }

        Ok(files_queue)
    }
}
