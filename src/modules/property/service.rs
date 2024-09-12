use std::sync::Arc;

use crate::{
    error::ApiError,
    utils::database::{Filter, FilterCondition, PaginatedRecord, Pagination},
};
use futures::stream::{self, StreamExt};
use uuid::Uuid;

use super::{
    port::{BucketRepository, DBRepository},
    Property, PropertyImage, PropertyWithImages,
};

pub struct Service {
    db_repo: Arc<dyn DBRepository>,
    bucket_repo: Arc<dyn BucketRepository>,
}
impl Service {
    pub fn new(db_repo: Arc<dyn DBRepository>, bucket_repo: Arc<dyn BucketRepository>) -> Self {
        Self {
            db_repo,
            bucket_repo,
        }
    }
}

impl Service {
    pub async fn create(
        &self,
        property: Property,
        images_urls: Vec<String>,
    ) -> Result<PropertyWithImages, ApiError> {
        let images = images_urls
            .into_iter()
            .map(|url| PropertyImage::new(property.id, &url, false))
            .collect();

        self.db_repo.create(property, images).await
    }

    pub async fn find_all_tenant_properties(
        &self,
        tenat_id: i32,
        pagination: Pagination,
    ) -> Result<PaginatedRecord<PropertyWithImages>, ApiError> {
        let mut filter = Filter::new();
        filter.add("tenant_id", FilterCondition::eq(tenat_id));

        self.db_repo.find_many(filter, pagination).await
    }

    pub async fn find_property_by_id(&self, id: i32) -> Result<PropertyWithImages, ApiError> {
        let mut filter = Filter::new();
        filter.add("id", FilterCondition::eq(id));
        self.db_repo.find(filter).await
    }

    pub async fn generate_post_presigned_urls(
        &self,
        tenant_id: i32,
        n_links: usize,
    ) -> Result<Vec<String>, ApiError> {
        let results: Vec<Result<String, ApiError>> = stream::iter(0..n_links)
            .map(|_| {
                let key = format!("tenant_{}/image_{}", tenant_id, Uuid::new_v4());
                async move { self.bucket_repo.post_presigned_url(&key).await }
            })
            .buffer_unordered(10)
            .collect()
            .await;

        results.into_iter().collect()
    }
}
