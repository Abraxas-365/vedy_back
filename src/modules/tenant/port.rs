use async_trait::async_trait;

use crate::error::ApiError;

use super::Tenant;

#[async_trait]
pub trait DBRepository: Send + Sync {
    async fn find_by_user_id(&self, id: &str) -> Result<Tenant, ApiError>;
}
