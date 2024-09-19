use std::time::Duration;

use async_trait::async_trait;
use futures::future::join_all;

use crate::{
    error::ApiError, modules::front::landing::feedback::port::BucketRepository,
    utils::s3::S3Repository,
};

#[async_trait]
impl BucketRepository for S3Repository {
    async fn post_presigned_url(&self, key: &str) -> Result<String, ApiError> {
        self.post_presigned_url(key, Duration::from_secs(120)).await
    }

    async fn delete_images(&self, images: &[String]) -> Result<Vec<String>, ApiError> {
        let delete_futures = images.iter().map(|image_url| {
            let client = self.clone();
            let image_url = image_url.clone();
            async move { client.delete_object(&image_url).await.map(|_| image_url) }
        });

        let results = join_all(delete_futures).await;

        let mut deleted_images = Vec::new();
        for result in results {
            match result {
                Ok(image_url) => deleted_images.push(image_url),
                Err(e) => return Err(e),
            }
        }

        Ok(deleted_images)
    }
}
