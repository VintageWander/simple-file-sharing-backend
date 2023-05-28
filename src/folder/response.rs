use crate::prisma::folder;

folder::select!(folder_response {
    id
    owner: select {
        id username email created_at updated_at
    }
    parent_folder_id
    folder_name
    visibility
    created_at
    updated_at
});
