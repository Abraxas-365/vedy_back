use async_trait::async_trait;

use crate::{error::ApiError, utils::database::Filter};

use super::Hero;

#[async_trait]
pub trait DBRepository: Send + Sync {
    async fn edit(&self, hero: Hero) -> Result<Hero, ApiError>;
    async fn find(&self, fileter: Filter) -> Result<Hero, ApiError>;
}

#[async_trait]
pub trait BucketRepository: Send + Sync {
    async fn post_presigned_url(&self, key: &str) -> Result<String, ApiError>;
}
