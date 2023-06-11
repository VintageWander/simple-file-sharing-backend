use axum::{extract::State, routing::delete, Router};

use crate::{
    user::model::{
        delete::DeleteUserRequest,
        loggedin::LoggedInUserWithPassword,
        select::{User, UserSelect},
    },
    validation::validation_message,
    web::Web,
    GlobalState, WebResult,
};

pub fn delete_user() -> Router<GlobalState> {
    async fn delete_user_handler(
        State(GlobalState {
            user_service,
            folder_service,
            ..
        }): State<GlobalState>,
        LoggedInUserWithPassword(User {
            id: user_id,
            password: user_password,
            ..
        }): LoggedInUserWithPassword,
        DeleteUserRequest { password, .. }: DeleteUserRequest,
    ) -> WebResult {
        if user_password != password {
            return Err(
                validation_message("Password provided does not match current password").into(),
            );
        }

        let UserSelect { id: owner_id, .. } = user_service.delete_user(user_id).await?;
        let deleted_folder = folder_service.delete_root_folder(owner_id).await?;

        Ok(Web::ok("Deleted user successfully", ()))
    }
    Router::new().route("/delete", delete(delete_user_handler))
}
