use axum::{extract::State, routing::post, Router};

use crate::{
    tag::model::create::CreateTagRequest,
    user::model::{loggedin::LoggedInUser, select::UserSelect},
    web::Web,
    GlobalState, WebResult,
};

pub fn create_tag() -> Router<GlobalState> {
    async fn create_tag_handler(
        State(GlobalState { tag_service, .. }): State<GlobalState>,
        LoggedInUser(UserSelect { id: user_id, .. }): LoggedInUser,
        CreateTagRequest { tag_name }: CreateTagRequest,
    ) -> WebResult {
        let new_tag = tag_service.create_tag(tag_name, user_id).await?;
        Ok(Web::created("Created new tag successfully", new_tag))
    }
    Router::new().route("/create", post(create_tag_handler))
}
