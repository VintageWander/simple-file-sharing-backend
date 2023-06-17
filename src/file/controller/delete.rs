use axum::{extract::State, routing::delete, Router};

use crate::{
    extractors::param::ParamId,
    prisma::file,
    user::model::{loggedin::LoggedInUser, select::UserSelect},
    web::Web,
    GlobalState, WebResult,
};

pub fn delete_file() -> Router<GlobalState> {
    async fn delete_file_handler(
        State(GlobalState {
            file_service,
            storage,
            ..
        }): State<GlobalState>,
        ParamId(file_id): ParamId,
        LoggedInUser(UserSelect { id: user_id, .. }): LoggedInUser,
    ) -> WebResult {
        let target_file = file_service
            .get_file_by_user_id(vec![file::id::equals(file_id)], user_id)
            .await?;

        let deleted_file = file_service.delete_file(target_file.id).await?;

        storage.delete_file(&deleted_file.id).await?;
        storage
            .delete_folder(&format!("{}/", deleted_file.id))
            .await?;

        Ok(Web::ok("Deleted file successfully", ()))
    }
    Router::new().route("/delete/:file_id", delete(delete_file_handler))
}
