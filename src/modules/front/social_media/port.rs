use async_trait::async_trait;

use crate::error::ApiError;

use super::SocialMedia;

#[async_trait]
pub trait DBRepository: Send + Sync {
    async fn upsert(
        &self,
        social_media: SocialMedia,
        tenant_id: i32,
    ) -> Result<SocialMedia, ApiError>;

    async fn find(&self, tenant_id: i32) -> Result<SocialMedia, ApiError>;
}
