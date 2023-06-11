use crate::prisma::user;

user::select!(user_select {
    id
    username
    email
    created_at
    updated_at
});

pub type User = user::Data;
pub type UserSelect = user_select::Data;
