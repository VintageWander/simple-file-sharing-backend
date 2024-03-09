use std::sync::Arc;

use crate::{
	error::Error,
	prisma::{file, folder, tag, user, PrismaClient},
};

use super::model::select::{tag_select, Tag, TagSelect};

#[derive(Clone)]
pub struct TagService {
	pub db: Arc<PrismaClient>,
}

impl TagService {
	pub fn init(db: &Arc<PrismaClient>) -> Self {
		Self { db: db.clone() }
	}

	pub async fn get_tags(
		&self,
		tag_id: Option<String>,
		tag_name: Option<String>,
		owner_id: Option<String>,
		file_id: Option<String>,
		folder_id: Option<String>,
	) -> Result<Vec<TagSelect>, Error> {
		let mut filters = vec![];

		if let Some(tag_id) = tag_id {
			filters.push(tag::id::equals(tag_id));
		}

		if let Some(tag_name) = tag_name {
			filters.push(tag::tag_name::equals(tag_name));
		}

		if let Some(owner_id) = owner_id {
			filters.push(tag::owner_id::equals(owner_id));
		}

		if let Some(file_id) = file_id {
			filters.push(tag::files::some(vec![file::id::equals(file_id)]));
		}

		if let Some(folder_id) = folder_id {
			filters.push(tag::folders::some(vec![folder::id::equals(folder_id)]));
		}

		let tags = self
			.db
			.tag()
			.find_many(filters)
			.select(tag_select::select())
			.exec()
			.await?;
		Ok(tags)
	}

	pub async fn get_owned_tag(&self, tag_id: String, owner_id: String) -> Result<Tag, Error> {
		let tag = self
			.db
			.tag()
			.find_first(vec![
				tag::id::equals(tag_id),
				tag::owner_id::equals(owner_id),
			])
			.exec()
			.await?
			.ok_or_else(|| Error::NotFound)?;
		Ok(tag)
	}

	pub async fn get_unique_tag(&self, tag_name: String, owner_id: String) -> Result<Tag, Error> {
		let tag = self
			.db
			.tag()
			.find_unique(tag::tag_name_owner_id(tag_name, owner_id))
			.exec()
			.await?
			.ok_or_else(|| Error::NotFound)?;

		Ok(tag)
	}

	pub async fn create_tag(&self, tag_name: String, owner_id: String) -> Result<TagSelect, Error> {
		let new_tag = self
			.db
			.tag()
			.create(tag_name, user::id::equals(owner_id), vec![])
			.select(tag_select::select())
			.exec()
			.await?;
		Ok(new_tag)
	}

	pub async fn update_tag(
		&self,
		tag_id: String,
		tag_name: Option<String>,
	) -> Result<TagSelect, Error> {
		let mut changes = vec![];
		if let Some(tag_name) = tag_name {
			changes.push(tag::tag_name::set(tag_name));
		}

		let updated_tag = self
			.db
			.tag()
			.update(tag::id::equals(tag_id), changes)
			.select(tag_select::select())
			.exec()
			.await?;
		Ok(updated_tag)
	}

	pub async fn delete_tag(&self, tag_id: String) -> Result<(), Error> {
		self.db.tag().delete(tag::id::equals(tag_id)).exec().await?;
		Ok(())
	}
}
