use std::sync::Arc;

use crate::error::ApiError;

use super::{port::DBRepository, LandingVisitedInfo, PropertyVisitedInfo, Stats};

pub struct Service {
    db_repo: Arc<dyn DBRepository>,
}

impl Service {
    pub fn new(db_repo: Arc<dyn DBRepository>) -> Self {
        Self { db_repo }
    }

    pub async fn create(&self, stats: Stats) -> Result<Stats, ApiError> {
        self.db_repo.create(stats).await
    }

    pub async fn get_property_visited_info(
        &self,
        tenant_id: i32,
    ) -> Result<Vec<PropertyVisitedInfo>, ApiError> {
        self.db_repo.get_property_visited_info(tenant_id).await
    }

    pub async fn get_landing_visited_info(
        &self,
        tenant_id: i32,
    ) -> Result<LandingVisitedInfo, ApiError> {
        self.db_repo.get_landing_visited_info(tenant_id).await
    }
}
