use crate::prisma::file;

file::select!(file_response {
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
