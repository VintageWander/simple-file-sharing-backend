use axum::{extract::State, routing::delete, Router};

use crate::{
    prisma::user::Data,
    user::request::{delete::DeleteUserRequest, loggedin::LoggedInUserWithPassword},
    validation::validation_message,
    web::Web,
    GlobalState, WebResult,
};

pub fn delete_user() -> Router<GlobalState> {
    async fn delete_user_handler(
        State(GlobalState { user_service, .. }): State<GlobalState>,
        LoggedInUserWithPassword(Data {
            id: user_id,
            password: user_password,
            ..
        }): LoggedInUserWithPassword,
        DeleteUserRequest { password, .. }: DeleteUserRequest,
    ) -> WebResult {
        match user_password == password {
            true => {
                user_service.delete_user(user_id).await?;
                Ok(Web::ok("Deleted user successfully", ()))
            }
            false => {
                Err(validation_message("Password provided does not match current password").into())
            }
        }
    }
    Router::new().route("/delete", delete(delete_user_handler))
}
