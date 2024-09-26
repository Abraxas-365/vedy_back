use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct SocialMedia {
    pub id: i32,
    pub tenant_id: i32,
    pub facebook_url: Option<String>,
    pub instagram_url: Option<String>,
    pub tiktok_url: Option<String>,
    pub linkedin_url: Option<String>,
}
