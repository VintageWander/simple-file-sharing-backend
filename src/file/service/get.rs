use chrono::{DateTime, FixedOffset};

use crate::{
    error::Error,
    file::model::select::{child_files_select, file_select, ChildFilesSelect, FileSelect},
    prisma::{file, folder, user, Extension, Visibility},
};

use super::FileService;

impl FileService {
    #[allow(clippy::too_many_arguments)]
    pub async fn get_child_files_from_folders(
        &self,
        id: Option<String>,
        owner_id: Option<String>,
        parent_folder_id: Option<String>,
        filename: Option<String>,
        extension: Option<Extension>,
        visibility: Option<Visibility>,
        created_at: Option<DateTime<FixedOffset>>,
        updated_at: Option<DateTime<FixedOffset>>,
    ) -> Result<Vec<ChildFilesSelect>, Error> {
        let starting_point = match parent_folder_id {
            Some(parent_id) => folder::id::equals(parent_id),
            None => folder::parent_folder_id::equals(None),
        };

        let mut child_files_filter = vec![];

        if let Some(id) = id.clone() {
            child_files_filter.push(file::id::equals(id))
        }

        if let Some(owner_id) = owner_id {
            child_files_filter.push(file::owner_id::equals(owner_id))
        }

        if let Some(filename) = filename.clone() {
            child_files_filter.push(file::filename::equals(filename))
        }

        if let Some(extension) = extension {
            child_files_filter.push(file::extension::equals(extension))
        }

        if let Some(visibility) = visibility {
            child_files_filter.push(file::visibility::equals(visibility))
        }

        if let Some(created_at) = created_at {
            child_files_filter.push(file::created_at::equals(created_at))
        }

        if let Some(updated_at) = updated_at {
            child_files_filter.push(file::updated_at::equals(updated_at))
        }

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

    #[allow(clippy::too_many_arguments)]
    pub async fn get_files_shared_to_user_id(
        &self,
        user_id: String,
        id: Option<String>,
        parent_folder_id: Option<String>,
        filename: Option<String>,
        extension: Option<Extension>,
        created_at: Option<DateTime<FixedOffset>>,
        updated_at: Option<DateTime<FixedOffset>>,
    ) -> Result<Vec<FileSelect>, Error> {
        let mut filters = vec![];

        if let Some(id) = id.clone() {
            filters.push(file::id::equals(id))
        }
        if let Some(filename) = filename.clone() {
            filters.push(file::filename::equals(filename))
        }

        if let Some(extension) = extension {
            filters.push(file::extension::equals(extension))
        }

        if let Some(created_at) = created_at {
            filters.push(file::created_at::equals(created_at))
        }

        if let Some(updated_at) = updated_at {
            filters.push(file::updated_at::equals(updated_at))
        }

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
