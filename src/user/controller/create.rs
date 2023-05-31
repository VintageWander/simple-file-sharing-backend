use axum::{extract::State, routing::post, Router};

use crate::{
    folder::response::folder_response,
    prisma::{folder, Visibility},
    user::request::create::CreateUserRequest,
    web::Web,
    GlobalState, WebResult,
};

pub fn create_user() -> Router<GlobalState> {
    async fn create_user_handler(
        State(GlobalState {
            db, user_service, ..
        }): State<GlobalState>,
        CreateUserRequest {
            username,
            email,
            password,
            ..
        }: CreateUserRequest,
    ) -> WebResult {
        let new_user = user_service.create_user(username, email, password).await?;

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
