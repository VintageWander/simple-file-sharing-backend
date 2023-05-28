use axum::{extract::State, routing::post, Router};

use crate::{
    folder::response::folder_response,
    prisma::{folder, Visibility},
    user::{request::create::CreateUserRequest, response::user_response},
    web::Web,
    GlobalState, WebResult,
};

pub fn create_user() -> Router<GlobalState> {
    async fn create_user_handler(
        State(GlobalState { db, .. }): State<GlobalState>,
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
            .select(user_response::select())
            .exec()
            .await?;

        db.folder()
            .create_unchecked(
                new_user.id.clone(),
                new_user.id.clone(),
                Visibility::Private,
                vec![folder::parent_folder_id::set(None)],
            )
            .select(folder_response::select())
            .exec()
            .await?;

        Ok(Web::ok("Created user successfully", new_user))
    }
    Router::new().route("/create", post(create_user_handler))
}
