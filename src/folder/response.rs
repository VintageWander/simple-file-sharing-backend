use crate::prisma::folder;

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
