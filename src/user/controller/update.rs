use axum::{extract::State, routing::put, Router};

use crate::{
    prisma::user,
    user::model::{loggedin::LoggedInUserWithPassword, update::UpdateUserRequest},
    validation::validation_message,
    web::Web,
    GlobalState, WebResult,
};

pub fn update_user() -> Router<GlobalState> {
    async fn update_user_handler(
        State(GlobalState { user_service, .. }): State<GlobalState>,
        LoggedInUserWithPassword(user::Data {
            id: user_id,
            password: user_password,
            ..
        }): LoggedInUserWithPassword,
        UpdateUserRequest {
            username,
            email,
            password,
            new_password,
            ..
        }: UpdateUserRequest,
    ) -> WebResult {
        if user_password != password {
            return Err(
                validation_message("Provided password does not match current password").into(),
            );
        }

        /*
        If all passed, set the queries

        This part of code does not embedded inside the request (extractor) logic
        Because I don't want any of this gets executed unless the passwords are valid
        */

        let mut changes = vec![];

        if let Some(username) = username {
            changes.push(user::username::set(username))
        }
        if let Some(email) = email {
            changes.push(user::email::set(email))
        }
        if let Some(new_password) = new_password {
            changes.push(user::password::set(new_password))
        }

        let updated_user = user_service.update_user(user_id, changes).await?;
        Ok(Web::ok("Update user successfully", updated_user))
    }
    Router::new().route("/update", put(update_user_handler))
}
