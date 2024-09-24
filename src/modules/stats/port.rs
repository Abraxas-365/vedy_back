use async_trait::async_trait;

use crate::error::ApiError;

use super::{LandingVisitedInfo, PropertyVisitedInfo, Stats};

#[async_trait]
pub trait DBRepository: Send + Sync {
    async fn create(&self, stats: Stats) -> Result<Stats, ApiError>;

    async fn get_property_visited_info(
        &self,
        tenant_id: i32,
    ) -> Result<Vec<PropertyVisitedInfo>, ApiError>;

    async fn get_landing_visited_info(
        &self,
        tenant_id: i32,
    ) -> Result<LandingVisitedInfo, ApiError>;
}
