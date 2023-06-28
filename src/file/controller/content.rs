use axum::{
    body::StreamBody,
    extract::{Query, State},
    http::header::{CONTENT_DISPOSITION, CONTENT_TYPE},
    response::{AppendHeaders, IntoResponse},
    routing::{get, post},
    Router,
};
use mime_guess::from_path;
use serde::Deserialize;
use tokio_util::io::ReaderStream;

use crate::{
    error::Error,
    extractors::param::ParamId,
    file::utils::encode_key,
    prisma::file,
    user::model::{loggedin::LoggedInUser, select::UserSelect},
    web::Web,
    GlobalState, WebResult,
};

pub fn get_file_content() -> Router<GlobalState> {
    #[derive(Deserialize)]
    #[serde(rename_all = "camelCase")]
    struct FileVersionQuery {
        version_number: Option<i64>,
        key: Option<String>,
    }

    async fn get_file_content_handler(
        State(GlobalState {
            file_service,
            storage,
            ..
        }): State<GlobalState>,
        user_or_guest: Option<LoggedInUser>,
        ParamId(file_id): ParamId,
        Query(FileVersionQuery {
            version_number,
            key,
        }): Query<FileVersionQuery>,
    ) -> WebResult {
        let found_file = match user_or_guest {
            Some(LoggedInUser(UserSelect { id: user_id, .. })) => {
                file_service
                    .get_file_by_user_id(vec![file::id::equals(file_id)], user_id)
                    .await
            }
            None => file_service.get_public_file_by_id(file_id).await,
        };

        let found_file = match found_file {
            Ok(found_file) => found_file,
            Err(_) => match key {
                Some(key) => file_service.get_file_from_key(key).await?,
                None => return Err(Error::NotFound),
            },
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
    Router::new().route("/content/:file_id", get(get_file_content_handler))
}

pub fn generate_file_temp_key() -> Router<GlobalState> {
    #[derive(Deserialize)]
    #[serde(rename_all = "camelCase")]
    struct FileExpiryQuery {
        expiry: Option<String>,
    }
    async fn generate_file_temp_key_handler(
        State(GlobalState { file_service, .. }): State<GlobalState>,
        LoggedInUser(UserSelect { id: user_id, .. }): LoggedInUser,
        ParamId(file_id): ParamId,
        Query(FileExpiryQuery { expiry }): Query<FileExpiryQuery>,
    ) -> WebResult {
        let sharable_file = file_service
            .get_file_by_user_id(vec![file::id::equals(file_id)], user_id)
            .await?;

        let key = encode_key(
            &sharable_file.id,
            expiry.unwrap_or_else(|| "2m".into()).into(),
        )?;

        Ok(Web::ok("Temp key to access file generated", key))
    }
    Router::new().route("/content/:file_id", post(generate_file_temp_key_handler))
}
