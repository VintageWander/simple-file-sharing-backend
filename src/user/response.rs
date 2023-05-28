use crate::prisma::user;

user::select!(user_response {
    id
    username
    email
    created_at
    updated_at
});
