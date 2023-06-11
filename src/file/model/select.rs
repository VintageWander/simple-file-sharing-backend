use crate::prisma::{
    file::{self, WhereParam},
    folder,
};

file::select!(file_select {
    id
    owner: select {
        id username email created_at updated_at
    }
    collaborators
    parent_folder_id

    filename
    extension
    visibility
    versions

    created_at
    updated_at
});

pub type File = file::Data;
pub type FileSelect = file_select::Data;

folder::select!((filters: Vec<WhereParam>) => child_files_select {
    child_files(filters): select {
        id
        owner: select {
            id username email created_at updated_at
        }
        collaborators
        parent_folder_id

        filename
        extension
        visibility

        created_at
        updated_at
    }
});

pub type ChildFilesSelect = child_files_select::child_files::Data;
