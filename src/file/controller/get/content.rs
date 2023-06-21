use axum::{
    body::StreamBody,
    extract::{Path, Query, State},
    http::header::{CONTENT_DISPOSITION, CONTENT_TYPE},
    response::{AppendHeaders, IntoResponse},
    routing::get,
    Router,
};
use mime_guess::from_path;
use serde::Deserialize;
use tokio_util::io::ReaderStream;

use crate::{
    error::Error,
    file::model::select::file_select,
    prisma::{file, Visibility},
    user::model::{loggedin::LoggedInUser, select::UserSelect},
    GlobalState, WebResult,
};

pub fn get_content() -> Router<GlobalState> {
    #[derive(Deserialize)]
    #[serde(rename_all = "camelCase")]
    struct FileVersionQuery {
        version_number: Option<i64>,
    }

    async fn get_content_handler(
        State(GlobalState {
            db,
            file_service,
            storage,
            ..
        }): State<GlobalState>,
        user_or_guest: Option<LoggedInUser>,
        Path(file_id): Path<String>,
        Query(FileVersionQuery { version_number }): Query<FileVersionQuery>,
    ) -> WebResult {
        let found_file = match user_or_guest {
            Some(LoggedInUser(UserSelect { id: user_id, .. })) => {
                file_service
                    .get_file_by_user_id(vec![file::id::equals(file_id)], user_id)
                    .await?
            }
            None => db
                .file()
                .find_first(vec![
                    file::id::equals(file_id),
                    file::visibility::equals(Visibility::Public),
                ])
                .select(file_select::select())
                .exec()
                .await?
                .ok_or_else(|| Error::NotFound)?,
        };

        let file_path = match version_number {
            Some(version_number) => format!(
                "{}/{}.{}",
                found_file.id,
                version_number,
                found_file.extension.to_string()
            ),
            None => format!("{}.{}", found_file.id, found_file.extension.to_string()),
        };

        let bytes = storage.get_data_by_key(&file_path).await?;

        let stream = ReaderStream::new(bytes.into_async_read());

        let body = StreamBody::new(stream);

        let full_filename = format!(
            "{}.{}",
            found_file.filename,
            found_file.extension.to_string()
        );

        let mime = from_path(&full_filename)
            .first_or_octet_stream()
            .to_string();

        Ok((
            AppendHeaders([
                (CONTENT_TYPE, mime),
                (
                    CONTENT_DISPOSITION,
                    format!("attachment; filename=\"{full_filename}\""),
                ),
            ]),
            body,
        )
            .into_response())
    }
    Router::new().route("/content/:file_id/", get(get_content_handler))
}