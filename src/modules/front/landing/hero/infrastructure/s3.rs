use std::time::Duration;

use async_trait::async_trait;

use crate::{
    error::ApiError, modules::front::landing::hero::port::BucketRepository, utils::s3::S3Repository,
};

#[async_trait]
impl BucketRepository for S3Repository {
    async fn post_presigned_url(&self, key: &str) -> Result<String, ApiError> {
        self.post_presigned_url(key, Duration::from_secs(120)).await
    }
}
