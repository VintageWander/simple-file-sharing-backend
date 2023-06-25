use axum::{extract::State, routing::put, Router};

use crate::{
    prisma::{file, folder, tag},
    tag::model::attach::AttachRequest,
    user::model::{loggedin::LoggedInUser, select::UserSelect},
    web::Web,
    GlobalState, WebResult,
};

pub fn attach() -> Router<GlobalState> {
    async fn attach_handler(
        State(GlobalState {
            tag_service,
            file_service,
            folder_service,
            ..
        }): State<GlobalState>,
        LoggedInUser(UserSelect { id: user_id, .. }): LoggedInUser,
        AttachRequest {
            tag_names,
            file_ids,
            folder_ids,
        }: AttachRequest,
    ) -> WebResult {
        let mut owned_tags = vec![];
        for tag_name in tag_names {
            owned_tags.push(
                tag_service
                    .get_unique_tag(tag_name, user_id.clone())
                    .await?,
            );
        }

        let owned_tags: Vec<_> = owned_tags
            .into_iter()
            .map(|tag| tag::id::equals(tag.id))
            .collect();

        for file_id in file_ids {
            file_service
                .db
                .file()
                .update(
                    file::id::equals(file_id),
                    vec![file::tags::set(owned_tags.clone())],
                )
                .exec()
                .await?;
        }

        for folder_id in folder_ids {
            folder_service
                .db
                .folder()
                .update(
                    folder::id::equals(folder_id),
                    vec![folder::tags::set(owned_tags.clone())],
                )
                .exec()
                .await?;
        }

        Ok(Web::ok(
            "Attach all requested tags to files and folders successfully!",
            (),
        ))
    }
    Router::new().route("/attach", put(attach_handler))
}
