use std::sync::Arc;

use crate::prisma::PrismaClient;

pub mod get;

#[derive(Clone)]
pub struct FileService {
    db: Arc<PrismaClient>,
}

impl FileService {
    pub fn init(db: &Arc<PrismaClient>) -> Self {
        Self { db: db.clone() }
    }
}
