use std::sync::Arc;

use uuid::Uuid;

use crate::{
    error::ApiError,
    utils::database::{Filter, FilterCondition},
};

use super::{
    port::{BucketRepository, DBRepository},
    Hero,
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
        let key = format!("tenants_hero/tenant_{}/image_{}", tenant_id, Uuid::new_v4());
        self.bucket_repo.post_presigned_url(&key).await
    }

    pub async fn update_hero(&self, config: Hero) -> Result<Hero, ApiError> {
        self.db_repo.edit(config).await
    }

    pub async fn find_tenant_hero(&self, tenant_id: i32) -> Result<Hero, ApiError> {
        let mut filter = Filter::new();
        filter.add("tenant_id", FilterCondition::eq(tenant_id));

        self.db_repo.find(filter).await
    }
}
