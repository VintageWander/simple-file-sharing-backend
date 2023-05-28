use chrono::{DateTime, FixedOffset};
use serde::Deserialize;
use validator::Validate;

use crate::{
    prisma::Visibility,
    validation::{file::check_folder_name, uuid::check_uuid},
};

#[derive(Deserialize, Validate)]
pub struct FolderQuery {
    #[validate(custom = "check_uuid")]
    pub id: Option<String>,

    #[validate(custom = "check_uuid")]
    pub owner_id: Option<String>,

    #[validate(custom = "check_uuid")]
    pub parent_folder_id: Option<String>,

    #[validate(custom = "check_folder_name")]
    pub folder_name: Option<String>,

    pub visibility: Option<Visibility>,

    pub created_at: Option<DateTime<FixedOffset>>,

    pub updated_at: Option<DateTime<FixedOffset>>,
}
