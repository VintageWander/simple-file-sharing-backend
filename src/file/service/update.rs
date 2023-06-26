use crate::{
    error::Error,
    file::model::select::{file_select, FileSelect},
    prisma::{file, tag, user, Visibility},
    tag::model::select::Tag,
    user::model::select::UserSelect,
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

    pub async fn set_collaborators_to_file(
        &self,
        file_id: String,
        collaborators: Vec<UserSelect>,
    ) -> Result<(), Error> {
        let collaborators = collaborators
            .into_iter()
            .map(|collaborator| user::id::equals(collaborator.id))
            .collect();

        self.db
            .file()
            .update(
                file::id::equals(file_id),
                vec![file::collaborators::set(collaborators)],
            )
            .exec()
            .await?;

        Ok(())
    }

    pub async fn set_tags_to_file(&self, tags: Vec<Tag>, file_id: String) -> Result<(), Error> {
        let tags = tags
            .into_iter()
            .map(|tag| tag::id::equals(tag.id))
            .collect();

        self.db
            .file()
            .update(file::id::equals(file_id), vec![file::tags::set(tags)])
            .exec()
            .await?;
        Ok(())
    }
}
