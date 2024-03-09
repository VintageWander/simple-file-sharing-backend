use std::fs::File;
use std::{collections::VecDeque, io::Write};

use axum::{
	body::Body,
	extract::State,
	http::header::{CONTENT_DISPOSITION, CONTENT_TYPE},
	response::{AppendHeaders, IntoResponse},
	routing::get,
	Router,
};
use tokio_util::io::ReaderStream;
use zip::{write::FileOptions, ZipWriter};

use crate::{
	extractors::param::ParamId,
	prisma::folder,
	user::model::{loggedin::LoggedInUser, select::UserSelect},
	GlobalState, WebResult,
};

pub fn get_folder_content() -> Router<GlobalState> {
	async fn get_folder_content_handler(
		State(GlobalState {
			folder_service,
			storage,
			..
		}): State<GlobalState>,
		user_or_guest: Option<LoggedInUser>,
		ParamId(folder_id): ParamId,
	) -> WebResult {
		// Find the folder from param id
		let found_folder = match user_or_guest {
			Some(LoggedInUser(UserSelect { id: user_id, .. })) => {
				folder_service
					.get_folder_by_user_id(vec![folder::id::equals(folder_id)], user_id)
					.await?
			}
			None => folder_service.get_public_folder_by_id(folder_id).await?,
		};

		let zip_location = format!("./src/downloads/{}.zip", &found_folder.id);

		// Initialize the zip writer
		let mut zip = ZipWriter::new(File::create(&zip_location)?);

		let mut path: String = "".into();

		// Create a folder_id_queue to perform a breadth first search
		let mut folder_id_queue = VecDeque::new();

		// Push the found_folder id in
		folder_id_queue.push_back(found_folder.id.clone());

		while let Some(folder_id) = folder_id_queue.pop_front() {
			// Inner join the child folders
			let folder = folder_service.get_folder_by_id(folder_id).await?;

			// Appends the folder name into the current path
			path.push_str(&format!("{}/", &folder.folder_name));

			// Add a directory at that position
			zip.add_directory(path.clone(), FileOptions::default())?;

			// Take the found_folder's child folders ids
			// Extends the folder_id_queue with the child folders ids
			folder_id_queue.extend(folder.child_folders.into_iter().map(|f| f.id));

			/*
				For each child files in the folder,
				Query the S3 storage for each of them
				and add them to the zip
			*/
			for file in folder.child_files {
				let real_file = storage
					.get_data_by_key(&format!("{}.{}", file.id, file.extension.to_string()))
					.await?;

				// Write to zip (if it's not empty)
				if let Ok(bytes) = real_file.collect().await.map(|data| data.into_bytes()) {
					zip.start_file(
						format!("{}/{}.{}", path, file.filename, file.extension.to_string()),
						FileOptions::default(),
					)?;
					zip.write_all(&bytes)?;
				}
			}
		}
		zip.finish()?;

		let full_zip = tokio::fs::File::open(&zip_location).await?;

		let bytes = tokio::io::BufReader::new(full_zip);

		let stream = ReaderStream::new(bytes);

		let body = Body::from_stream(stream);

		// Delete the file
		// We can do this because the entire zip file has been loaded to the StreamBody
		tokio::fs::remove_file(zip_location).await?;

		Ok((
			AppendHeaders([
				(CONTENT_TYPE, "application/zip".to_string()),
				(CONTENT_DISPOSITION, format!("{}.zip", found_folder.id)),
			]),
			body,
		)
			.into_response())
	}
	Router::new().route("/content/:folder_id", get(get_folder_content_handler))
}
