pub mod create;
pub mod delete;
pub mod get;
pub mod update;

use std::sync::Arc;

use crate::prisma::PrismaClient;

#[derive(Clone)]
pub struct FolderService {
	pub db: Arc<PrismaClient>,
}

impl FolderService {
	pub fn init(db: &Arc<PrismaClient>) -> Self {
		Self { db: db.clone() }
	}
}
