use crate::prisma::user;

user::select!(user_select {
    id
    username
    email
    created_at
    updated_at
});

user::select!(user_select_with_password {
    id
    password
});

pub type User = user::Data;
pub type UserSelect = user_select::Data;
pub type UserSelectWithPassword = user_select_with_password::Data;
