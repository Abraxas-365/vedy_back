use std::sync::Arc;

use uuid::Uuid;

use crate::{
    error::ApiError,
    utils::database::{Filter, FilterCondition, PaginatedRecord, Pagination},
};

use super::{
    port::{BucketRepository, DBRepository},
    Feedback,
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
    pub async fn generate_post_presigned_urls(&self, tenant_id: i32) -> Result<String, ApiError> {
        let key = format!("/tenant_{}/feedback/image_{}", tenant_id, Uuid::new_v4());
        self.bucket_repo.post_presigned_url(&key).await
    }
    pub async fn update_feedback(&self, feedback: Feedback) -> Result<Feedback, ApiError> {
        self.db_repo.edit(feedback).await
    }

    pub async fn create_feedback(&self, feedback: Feedback) -> Result<Feedback, ApiError> {
        self.db_repo.create(feedback).await
    }

    pub async fn find_tenant_feedback(
        &self,
        tenant_id: i32,
        pagination: Pagination,
    ) -> Result<PaginatedRecord<Feedback>, ApiError> {
        let mut filter = Filter::new();
        filter.add("tenant_id", FilterCondition::eq(tenant_id));
        self.db_repo.find_many(filter, pagination).await
    }
}
