use axum::{extract::State, routing::put, Router};

use crate::{
    tag::model::set::SetTagRequest,
    user::model::{loggedin::LoggedInUser, select::UserSelect},
    web::Web,
    GlobalState, WebResult,
};

pub fn set() -> Router<GlobalState> {
    async fn set_handler(
        State(GlobalState {
            tag_service,
            file_service,
            folder_service,
            ..
        }): State<GlobalState>,
        LoggedInUser(UserSelect { id: user_id, .. }): LoggedInUser,
        SetTagRequest {
            tag_names,
            file_ids,
            folder_ids,
        }: SetTagRequest,
    ) -> WebResult {
        let mut owned_tags = vec![];
        for tag_name in tag_names {
            owned_tags.push(
                tag_service
                    .get_unique_tag(tag_name, user_id.clone())
                    .await?,
            );
        }

        for file_id in file_ids {
            file_service
                .set_tags_to_file(owned_tags.clone(), file_id)
                .await?;
        }

        for folder_id in folder_ids {
            folder_service
                .set_tags_to_folder(owned_tags.clone(), folder_id)
                .await?;
        }

        Ok(Web::ok(
            "Set all requested tags to files and folders successfully!",
            (),
        ))
    }
    Router::new().route("/set", put(set_handler))
}
