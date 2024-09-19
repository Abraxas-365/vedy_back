use async_trait::async_trait;

use crate::{
    error::ApiError,
    utils::database::{Filter, PaginatedRecord, Pagination},
};

use super::{Property, PropertyImage, PropertyWithImages};

#[async_trait]
pub trait DBRepository: Send + Sync {
    async fn create(
        &self,
        property: Property,
        images: Vec<PropertyImage>,
    ) -> Result<PropertyWithImages, ApiError>;

    async fn edit_property_images(
        &self,
        property_id: i32,
        images: Vec<PropertyImage>,
    ) -> Result<Vec<PropertyImage>, ApiError>;

    async fn update_property(
        &self,
        id: i32,
        property: Property,
    ) -> Result<PropertyWithImages, ApiError>;

    async fn find(&self, filter: Filter) -> Result<PropertyWithImages, ApiError>;

    async fn find_many(
        &self,
        filter: Filter,
        pagination: Pagination,
    ) -> Result<PaginatedRecord<PropertyWithImages>, ApiError>;

    async fn delete(&self, id: i32, tenant_id: i32) -> Result<PropertyWithImages, ApiError>;
}

#[async_trait]
pub trait BucketRepository: Send + Sync {
    async fn post_presigned_url(&self, key: &str) -> Result<String, ApiError>;
    async fn delete_images(&self, images: &[String]) -> Result<Vec<String>, ApiError>;
}
