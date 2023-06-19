use axum::{extract::State, routing::put, Router};
use axum_typed_multipart::{FieldData, FieldMetadata};

use crate::{
    extractors::param::ParamId,
    file::model::{select::FileSelect, update::UpdateFileRequest},
    file_version::model::FileVersionSelect,
    prisma::{file, folder},
    user::model::{loggedin::LoggedInUser, select::UserSelect},
    validation::validation_message,
    web::Web,
    GlobalState, WebResult,
};

pub fn update_file() -> Router<GlobalState> {
    async fn update_file_handler(
        State(GlobalState {
            db,
            folder_service,
            file_service,
            file_version_service,
            storage,
            ..
        }): State<GlobalState>,
        ParamId(file_id): ParamId,
        LoggedInUser(UserSelect { id: user_id, .. }): LoggedInUser,
        UpdateFileRequest {
            parent,
            visibility,
            file,
        }: UpdateFileRequest,
    ) -> WebResult {
        let FileSelect {
            id: target_id,
            parent_folder_id: target_parent,
            filename: target_filename,
            ..
        } = file_service
            .get_file_by_user_id(vec![file::id::equals(file_id)], user_id.clone())
            .await?;

        let parent_folder_id = match parent {
            Some(parent) => {
                let parent_folder = folder_service
                    .get_folder_by_user_id(vec![folder::id::equals(parent)], user_id)
                    .await?;
                Some(parent_folder.id)
            }
            None => None,
        };

        let (filename, data) = match file {
            Some(FieldData {
                metadata: FieldMetadata { file_name, .. },
                contents,
            }) => {
                let Some(filename) = file_name else {
                    return Err(validation_message("The file uploaded must have a name").into())
                };
                (Some(filename), Some(contents))
            }
            None => (None, None),
        };

        let updated_file = file_service
            .update_file(target_id, parent_folder_id, filename, visibility)
            .await?;

        if let Some(data) = data {
            let FileVersionSelect {
                file: updated_file,
                version_number,
            } = file_version_service
                .create_version_for_file(updated_file.id)
                .await?;

            /*
                Move the current file from
                hello.txt
                to
                hello/123.txt
            */
            storage
                .move_file(
                    &format!("{}.{}", updated_file.id, updated_file.extension.to_string()),
                    &format!(
                        "{}/{}.{}",
                        updated_file.id,
                        version_number,
                        updated_file.extension.to_string()
                    ),
                )
                .await?;

            /*
                Use the bytes data to create a new file
                Replacing that previous file
            */
            storage
                .create_file(
                    &format!("{}.{}", updated_file.id, updated_file.extension.to_string()),
                    data,
                )
                .await?;

            return Ok(Web::ok("Updated file successfully", updated_file));
        }
        Ok(Web::ok("Updated file successfully", updated_file))
    }
    Router::new().route("/update/:file_id", put(update_file_handler))
}
