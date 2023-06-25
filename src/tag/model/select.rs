use crate::prisma::tag;

tag::select!(tag_select {
    id
    tag_name
    owner: select {
        id username email created_at updated_at
    }
    files: select {
        id
        owner: select {
            id username email created_at updated_at
        }
        filename
        extension
        visibility

        created_at
        updated_at
    }
    folders: select {
        id
        owner: select {
            id username email created_at updated_at
        }
        folder_name
        visibility

        created_at
        updated_at
    }
});

pub type Tag = tag::Data;
pub type TagSelect = tag_select::Data;
