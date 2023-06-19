use axum::{extract::State, routing::post, Router};

use crate::{user::model::create::CreateUserRequest, web::Web, GlobalState, WebResult};

pub fn create_user() -> Router<GlobalState> {
    async fn create_user_handler(
        State(GlobalState {
            db,
            user_service,
            folder_service,
            ..
        }): State<GlobalState>,
        CreateUserRequest {
            username,
            email,
            password,
            ..
        }: CreateUserRequest,
    ) -> WebResult {
        let new_user = user_service.create_user(username, email, password).await?;

        folder_service
            .create_root_folder(new_user.id.clone())
            .await?;

        Ok(Web::created("Created user successfully", new_user))
    }
    Router::new().route("/create", post(create_user_handler))
}
