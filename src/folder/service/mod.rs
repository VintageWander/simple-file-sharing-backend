pub mod get;

use std::sync::Arc;

use crate::prisma::PrismaClient;

#[derive(Clone)]
pub struct FolderService {
    db: Arc<PrismaClient>,
}

impl FolderService {
    pub fn init(db: &Arc<PrismaClient>) -> Self {
        Self { db: db.clone() }
    }
}
