use async_trait::async_trait;

use crate::{error::ApiError, utils::database::Filter};

use super::Config;

#[async_trait]
pub trait DBRepository {
    async fn edit(&self, config: Config) -> Result<Config, ApiError>;
    async fn find(&self, fileter: Filter) -> Result<Config, ApiError>;
}

#[async_trait]
pub trait BucketRepository: Send + Sync {
    async fn post_presigned_url(&self, key: &str) -> Result<String, ApiError>;
}
