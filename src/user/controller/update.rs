use axum::{extract::State, routing::put, Router};

use crate::{
    prisma::user::{self, Data},
    user::{
        request::{loggedin::LoggedInUser, update::UpdateUserRequest},
        response::user_response,
    },
    validation::validation_message,
    web::Web,
    GlobalState, WebResult,
};

pub fn update_user() -> Router<GlobalState> {
    async fn update_user_handler(
        State(GlobalState { db, .. }): State<GlobalState>,
        LoggedInUser(Data {
            id,
            password: user_password,
            ..
        }): LoggedInUser,
        UpdateUserRequest {
            username,
            email,
            password,
            new_password,
            ..
        }: UpdateUserRequest,
    ) -> WebResult {
        let mut actions = vec![];

        if user_password != password {
            return Err(
                validation_message("Provided password does not match current password").into(),
            );
        }

        if let Some(username) = username {
            actions.push(user::username::set(username))
        }
        if let Some(email) = email {
            actions.push(user::email::set(email))
        }
        if let Some(new_password) = new_password {
            actions.push(user::password::set(new_password))
        }

        let updated_user = db
            .user()
            .update(user::id::equals(id), actions)
            .select(user_response::select())
            .exec()
            .await?;
        Ok(Web::ok("Update user successfully", updated_user))
    }
    Router::new().route("/update", put(update_user_handler))
}
