use std::sync::Arc;

use crate::error::ApiError;

use super::{port::DBRepository, SocialMedia};

pub struct Service {
    db_repo: Arc<dyn DBRepository>,
}
impl Service {
    pub fn new(db_repo: Arc<dyn DBRepository>) -> Self {
        Self { db_repo }
    }

    pub async fn upsert(
        &self,
        social_media: SocialMedia,
        tenant_id: i32,
    ) -> Result<SocialMedia, ApiError> {
        self.db_repo.upsert(social_media, tenant_id).await
    }

    pub async fn find(&self, tenant_id: i32) -> Result<SocialMedia, ApiError> {
        self.db_repo.find(tenant_id).await
    }
}
