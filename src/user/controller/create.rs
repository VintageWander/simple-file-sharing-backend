use axum::{extract::State, routing::post, Router};

use crate::{
    prisma::{folder, user, Visibility},
    user::request::create::CreateUserRequest,
    web::Web,
    Database, WebResult,
};

use super::UserController;

impl UserController {
    pub fn create_user(&self) -> Router<Database> {
        async fn create_user_handler(
            State(db): State<Database>,
            CreateUserRequest {
                username,
                email,
                password,
                ..
            }: CreateUserRequest,
        ) -> WebResult {
            let new_user = db
                .user()
                .create(username, email, password, vec![])
                .select(user::select!({
                    id username email created_at updated_at
                }))
                .exec()
                .await?;

            db.folder()
                .create(
                    user::id::equals(new_user.id.clone()),
                    new_user.id.clone(),
                    Visibility::Private,
                    vec![folder::parent_folder_id::set(Some(new_user.id.clone()))],
                )
                .exec()
                .await?;
            Ok(Web::ok("Created user successfully", new_user))
        }
        Router::new().route("/create", post(create_user_handler))
    }
}
