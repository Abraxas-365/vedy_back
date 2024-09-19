use async_trait::async_trait;

use crate::{
    error::ApiError,
    utils::database::{Filter, PaginatedRecord, Pagination},
};

use super::Feedback;

#[async_trait]
pub trait DBRepository: Send + Sync {
    async fn create(&self, feedback: Feedback) -> Result<Feedback, ApiError>;
    async fn edit(&self, feedback: Feedback) -> Result<Feedback, ApiError>;
    async fn find_many(
        &self,
        filter: Filter,
        pagination: Pagination,
    ) -> Result<PaginatedRecord<Feedback>, ApiError>;
    async fn delete(&self, id: i32, tenant_id: i32) -> Result<Feedback, ApiError>;
}

#[async_trait]
pub trait BucketRepository: Send + Sync {
    async fn post_presigned_url(&self, key: &str) -> Result<String, ApiError>;
    async fn delete_images(&self, images: &[String]) -> Result<Vec<String>, ApiError>;
}
