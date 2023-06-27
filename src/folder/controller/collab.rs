use std::collections::VecDeque;

use axum::{extract::State, routing::put, Router};

use crate::{
    folder::model::collab::SetFolderCollabRequest,
    prisma::folder,
    user::model::{loggedin::LoggedInUser, select::UserSelect},
    web::Web,
    GlobalState, WebResult,
};

pub fn set_folder_collaborators() -> Router<GlobalState> {
    async fn set_folder_collaborators_handler(
        State(GlobalState {
            folder_service,
            file_service,
            user_service,
            ..
        }): State<GlobalState>,
        LoggedInUser(UserSelect { id: user_id, .. }): LoggedInUser,
        SetFolderCollabRequest {
            folder_id,
            user_ids,
        }: SetFolderCollabRequest,
    ) -> WebResult {
        let target_folder = folder_service
            .get_folder_by_user_id(vec![folder::id::equals(folder_id)], user_id.clone())
            .await?;

        let mut collaborators = vec![];
        for id in user_ids {
            collaborators.push(user_service.get_user_by_id(id).await?)
        }

        let mut folder_id_queue = VecDeque::new();
        let mut file_id_queue = VecDeque::new();

        folder_id_queue.push_back(target_folder.id);

        while !folder_id_queue.is_empty() {
            let folder_id = folder_id_queue.pop_front().expect("This should not error");
            let folder = folder_service
                .get_folder_by_user_id(vec![folder::id::equals(folder_id)], user_id.clone())
                .await?;

            let folder = folder_service.get_folder_by_id(folder.id).await?;

            folder_service
                .set_collaborators_to_folder(folder.id, collaborators.clone())
                .await?;

            folder_id_queue.extend(
                folder
                    .child_folders
                    .into_iter()
                    .map(|child_folder| child_folder.id),
            );

            file_id_queue.extend(
                folder
                    .child_files
                    .into_iter()
                    .map(|child_file| child_file.id),
            )
        }

        for file_id in file_id_queue {
            file_service
                .set_collaborators_to_file(file_id, collaborators.clone())
                .await?;
        }

        Ok(Web::ok(
            "Set collaborators to folder, and inner folders and files successfully",
            (),
        ))
    }
    Router::new().route("/collaborators", put(set_folder_collaborators_handler))
}
