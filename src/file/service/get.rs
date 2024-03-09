use chrono::{DateTime, FixedOffset};

use crate::{
	error::Error,
	file::{
		model::select::{child_files_select, file_select, ChildFilesSelect, FileSelect},
		utils::decode_key,
	},
	prisma::{
		file::{self, WhereParam},
		folder, user, Extension, Visibility,
	},
};

use super::FileService;

impl FileService {
	#[allow(clippy::too_many_arguments)]
	pub async fn get_child_files_from_folders(
		&self,
		id: Option<String>,
		owner_id: Option<String>,
		parent_folder_id: Option<String>,
		filename: Option<String>,
		extension: Option<Extension>,
		visibility: Option<Visibility>,
		created_at: Option<DateTime<FixedOffset>>,
		updated_at: Option<DateTime<FixedOffset>>,
	) -> Result<Vec<ChildFilesSelect>, Error> {
		let starting_point = match parent_folder_id {
			Some(parent_id) => folder::id::equals(parent_id),
			None => folder::parent_folder_id::equals(None),
		};

		let mut child_files_filter = vec![];

		if let Some(id) = id {
			child_files_filter.push(file::id::equals(id))
		}

		if let Some(owner_id) = owner_id {
			child_files_filter.push(file::owner_id::equals(owner_id))
		}

		if let Some(filename) = filename {
			child_files_filter.push(file::filename::equals(filename))
		}

		if let Some(extension) = extension {
			child_files_filter.push(file::extension::equals(extension))
		}

		if let Some(visibility) = visibility {
			child_files_filter.push(file::visibility::equals(visibility))
		}

		if let Some(created_at) = created_at {
			child_files_filter.push(file::created_at::equals(created_at))
		}

		if let Some(updated_at) = updated_at {
			child_files_filter.push(file::updated_at::equals(updated_at))
		}

		let child_files = self
			.db
			.folder()
			.find_many(vec![starting_point])
			.select(child_files_select::select(child_files_filter))
			.exec()
			.await?
			.into_iter()
			.flat_map(|parent_folder| parent_folder.child_files)
			.collect();

		Ok(child_files)
	}

	pub async fn get_file_by_user_id(
		&self,
		file_filter: Vec<WhereParam>,
		user_id: String,
	) -> Result<FileSelect, Error> {
		/*
			There are 2 things that we have to deal with, that is
			We'll use the file_filter to get the file
			Then prepare 2 queries,
			1. Search the file by owner_id
			2. Search the file's collaborators list to see if there's one user_id equals to us

			We'll execute the first one, if it returns true, then return the file
			If it returns false, try the second one, if that is true, then return the file
			Finally, if both fails, we return a NotFound error

			The operation will cost at most 2 queries, and each query returns only one file
		*/
		let user_id_filter = file_filter
			.clone()
			.into_iter()
			.chain(vec![file::owner_id::equals(user_id.clone())])
			.collect();

		let collaborator_filter = file_filter
			.into_iter()
			.chain(vec![
				file::visibility::equals(Visibility::Shared),
				file::collaborators::some(vec![user::id::equals(user_id)]),
			])
			.collect();

		let search_by_user_id = self
			.db
			.file()
			.find_first(user_id_filter)
			.select(file_select::select())
			.exec();

		let search_from_collaborators = self
			.db
			.file()
			.find_first(collaborator_filter)
			.select(file_select::select())
			.exec();

		match search_by_user_id.await? {
			Some(owned_file) => Ok(owned_file),
			None => match search_from_collaborators.await? {
				Some(shared_to_user_file) => Ok(shared_to_user_file),
				None => Err(Error::NotFound),
			},
		}
	}

	#[allow(clippy::too_many_arguments)]
	pub async fn get_files_shared_to_user_id(
		&self,
		user_id: String,
		id: Option<String>,
		parent_folder_id: Option<String>,
		filename: Option<String>,
		extension: Option<Extension>,
		created_at: Option<DateTime<FixedOffset>>,
		updated_at: Option<DateTime<FixedOffset>>,
	) -> Result<Vec<FileSelect>, Error> {
		let mut filters = vec![];

		if let Some(id) = id {
			filters.push(file::id::equals(id))
		}

		if let Some(filename) = filename {
			filters.push(file::filename::equals(filename))
		}

		if let Some(parent) = parent_folder_id {
			filters.push(file::parent_folder_id::equals(parent))
		}

		if let Some(extension) = extension {
			filters.push(file::extension::equals(extension))
		}

		if let Some(created_at) = created_at {
			filters.push(file::created_at::equals(created_at))
		}

		if let Some(updated_at) = updated_at {
			filters.push(file::updated_at::equals(updated_at))
		}

		filters.extend(vec![
			file::visibility::equals(Visibility::Shared),
			file::collaborators::some(vec![user::id::equals(user_id)]),
		]);

		let shared_files = self
			.db
			.file()
			.find_many(filters)
			.select(file_select::select())
			.exec()
			.await?;
		Ok(shared_files)
	}

	pub async fn get_public_file_by_id(&self, file_id: String) -> Result<FileSelect, Error> {
		let public_file = self
			.db
			.file()
			.find_first(vec![
				file::id::equals(file_id),
				file::visibility::equals(Visibility::Public),
			])
			.select(file_select::select())
			.exec()
			.await?
			.ok_or_else(|| Error::NotFound)?;
		Ok(public_file)
	}

	pub async fn get_file_from_key(&self, key: String) -> Result<FileSelect, Error> {
		let file_id = decode_key(key)?;
		let file = self
			.db
			.file()
			.find_unique(file::id::equals(file_id))
			.select(file_select::select())
			.exec()
			.await?
			.ok_or_else(|| Error::NotFound)?;
		Ok(file)
	}
}
