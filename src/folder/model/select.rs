use crate::prisma::folder::{self, WhereParam};

folder::select!(folder_select {
    id
    owner: select {
        id username email created_at updated_at
    }
    parent_folder_id
    collaborators
    folder_name
    visibility
    tags
    created_at
    updated_at
});

folder::select!(folder_owner_select {
    id
    owner: select {
        id
    }
});

pub type Folder = folder::Data;
pub type FolderOwnerSelect = folder_owner_select::Data;
pub type FolderSelect = folder_select::Data;

folder::select!((filters: Vec<WhereParam>) => child_folders_select {
    child_folders(filters): select {
        id
        owner: select {
            id username email created_at updated_at
        }
        parent_folder_id
        folder_name
        visibility
        created_at
        updated_at
    }
});

pub type ChildFoldersSelect = child_folders_select::child_folders::Data;
