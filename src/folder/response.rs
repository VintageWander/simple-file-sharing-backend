use crate::prisma::folder::{self, WhereParam};

folder::select!(folder_response {
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

folder::select!((filters: Vec<WhereParam>) => child_folders_response {
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
