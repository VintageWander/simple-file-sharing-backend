use std::collections::VecDeque;

use chrono::{DateTime, FixedOffset};

use crate::{
	error::Error,
	folder::model::select::{
		child_folders_select, folder_select, ChildFoldersSelect, FolderSelect,
	},
	prisma::{
		folder::{self, WhereParam},
		user, Extension, Visibility,
	},
};

folder::select!(child_files_folders {
	id
	folder_name
	child_files: select {
		id
		filename
		extension
	}
	child_folders: select {
		id
	}
});

use super::FolderService;

impl FolderService {
	/*
		This function returns child folders from a list of root folders (or one folder, based on the first param)

		param parent_folder_id to find specify which folder should be the parent
		If Some -> the id will be used to get that folder
		If None -> get all root folders

		The rest of the fields are used as filters for the child folders
	*/
	#[allow(clippy::too_many_arguments)]
	pub async fn get_child_folders_from_folders(
		&self,
		id: Option<String>,
		owner_id: Option<String>,
		parent_folder_id: Option<String>,
		folder_name: Option<String>,
		visibility: Option<Visibility>,
		created_at: Option<DateTime<FixedOffset>>,
		updated_at: Option<DateTime<FixedOffset>>,
	) -> Result<Vec<ChildFoldersSelect>, Error> {
		//
		let starting_point = match parent_folder_id {
			Some(parent_id) => folder::id::equals(parent_id),
			None => folder::parent_folder_id::equals(None),
		};

		let mut child_folders_filter = vec![];

		if let Some(id) = id {
			child_folders_filter.push(folder::id::equals(id))
		};

		if let Some(owner_id) = owner_id {
			child_folders_filter.push(folder::owner_id::equals(owner_id))
		};

		if let Some(folder_name) = folder_name {
			child_folders_filter.push(folder::folder_name::equals(folder_name))
		};

		if let Some(visibility) = visibility {
			child_folders_filter.push(folder::visibility::equals(visibility))
		};

		if let Some(created_at) = created_at {
			child_folders_filter.push(folder::created_at::equals(created_at))
		}

		if let Some(updated_at) = updated_at {
			child_folders_filter.push(folder::updated_at::equals(updated_at))
		}

		let child_folders = self
			.db
			.folder()
			.find_many(vec![starting_point])
			.select(child_folders_select::select(child_folders_filter))
			.exec()
			.await?
			.into_iter()
			.flat_map(|parent_folder| parent_folder.child_folders)
			.collect();
		Ok(child_folders)
	}

	// Get all "Shared to me" folders, by user_id
	pub async fn get_folders_shared_to_user_id(
		&self,
		user_id: String,
		id: Option<String>,
		parent_folder_id: Option<String>,
		folder_name: Option<String>,
		created_at: Option<DateTime<FixedOffset>>,
		updated_at: Option<DateTime<FixedOffset>>,
	) -> Result<Vec<FolderSelect>, Error> {
		let mut filters = vec![];

		if let Some(id) = id {
			filters.push(folder::id::equals(id))
		}

		if let Some(parent_folder_id) = parent_folder_id {
			filters.push(folder::parent_folder_id::equals(Some(parent_folder_id)))
		}

		if let Some(folder_name) = folder_name {
			filters.push(folder::folder_name::equals(folder_name))
		}

		if let Some(created_at) = created_at {
			filters.push(folder::created_at::equals(created_at))
		}

		if let Some(updated_at) = updated_at {
			filters.push(folder::updated_at::equals(updated_at))
		}

		filters.extend(vec![
			folder::visibility::equals(Visibility::Shared),
			folder::collaborators::some(vec![user::id::equals(user_id)]),
		]);

		let shared_folders = self
			.db
			.folder()
			.find_many(filters)
			.select(folder_select::select())
			.exec()
			.await?;
		Ok(shared_folders)
	}

	pub async fn get_folder_by_user_id(
		&self,
		folder_filter: Vec<WhereParam>,
		user_id: String,
	) -> Result<FolderSelect, Error> {
		/*
			There are 2 things that we have to deal with, that is
			We'll use the folder_filter to get the folder
			Then prepare 2 queries,
			1. Search the folder by owner_id
			2. Search the folder's collaborators list to see if there's one user_id equals to us

			We'll execute the first one, if it returns true, then return the folder
			If it returns false, try the second one, if that is true, then return the folder
			Finally, if both fails, we return a NotFound error

			The operation will cost at most 2 queries, and each query returns only one folder
		*/

		let user_id_filter = folder_filter
			.clone()
			.into_iter()
			.chain(vec![folder::owner_id::equals(user_id.clone())])
			.collect();

		let collaborator_filter = folder_filter
			.into_iter()
			.chain(vec![
				folder::visibility::equals(Visibility::Shared),
				folder::collaborators::some(vec![user::id::equals(user_id)]),
			])
			.collect();

		let search_by_user_id = self
			.db
			.folder()
			.find_first(user_id_filter)
			.select(folder_select::select())
			.exec();

		let search_from_collaborators = self
			.db
			.folder()
			.find_first(collaborator_filter)
			.select(folder_select::select())
			.exec();

		match search_by_user_id.await? {
			Some(owned_folder) => Ok(owned_folder),
			None => match search_from_collaborators.await? {
				Some(shared_to_user_folder) => Ok(shared_to_user_folder),
				None => Err(Error::NotFound),
			},
		}
	}

	/*
		This function does get a folder based on its id
		but what it returns is not the folder itself
		It returns the folder's child folders and child files

		If you want to get the folder by id, you can use get_folder_by_user_id
		Performing a query for a dedicated folder MUST associate with a user_id
		This is to ensure that no private or shared folder be leaked accidentally to the public controller
	*/
	pub async fn get_folder_by_id(
		&self,
		folder_id: String,
	) -> Result<child_files_folders::Data, Error> {
		let folder = self
			.db
			.folder()
			.find_unique(folder::id::equals(folder_id))
			.select(child_files_folders::select())
			.exec()
			.await?
			.ok_or_else(|| Error::NotFound)?;
		Ok(folder)
	}

	/*
		What this function does is that it gets all files that lives under the folder_id
		Even child files, grandchild files, grandgrandchild files, all of them
	*/
	pub async fn get_nested_files_from_folder(
		&self,
		folder_id: String,
	) -> Result<VecDeque<(String, Extension)>, Error> {
		let mut folder_id_queue = VecDeque::new();
		let mut file_id_queue = VecDeque::new();

		folder_id_queue.push_back(folder_id);

		while let Some(first_folder_id) = folder_id_queue.pop_front() {
			let first_folder = self.get_folder_by_id(first_folder_id).await?;

			folder_id_queue.extend(first_folder.child_folders.into_iter().map(|f| f.id));

			file_id_queue.extend(
				first_folder
					.child_files
					.into_iter()
					.map(|f| (f.id, f.extension)),
			);
		}

		Ok(file_id_queue)
	}

	pub async fn get_root_folder(&self, owner_id: String) -> Result<String, Error> {
		let root_folder = self
			.db
			.folder()
			.find_first(vec![
				folder::owner_id::equals(owner_id.clone()),
				folder::folder_name::equals(owner_id),
				folder::visibility::equals(Visibility::Private),
				folder::parent_folder_id::equals(None),
			])
			.select(folder::select!({ id }))
			.exec()
			.await?
			.ok_or_else(|| Error::NotFound)?
			.id;

		Ok(root_folder)
	}

	pub async fn get_public_folder_by_id(&self, folder_id: String) -> Result<FolderSelect, Error> {
		let public_folder = self
			.db
			.folder()
			.find_first(vec![
				folder::visibility::equals(Visibility::Public),
				folder::id::equals(folder_id),
			])
			.select(folder_select::select())
			.exec()
			.await?
			.ok_or_else(|| Error::NotFound)?;
		Ok(public_folder)
	}
}
