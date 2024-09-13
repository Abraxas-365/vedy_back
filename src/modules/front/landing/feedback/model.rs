use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

#[derive(Debug, Clone, Deserialize, Serialize, FromRow)]
pub struct Feedback {
    pub id: i32,
    pub tenant_id: i32,
    pub property_image: String,
    pub customer_image: String,
    pub customer_name: String,
    pub customer_review: String,
    pub description: String,
}
impl Feedback {
    pub fn new(
        tenant_id: i32,
        property_image: &str,
        customer_image: &str,
        customer_name: &str,
        customer_review: &str,
        description: &str,
    ) -> Self {
        Self {
            id: 0,
            tenant_id,
            property_image: property_image.into(),
            customer_image: customer_image.into(),
            customer_name: customer_name.into(),
            customer_review: customer_review.into(),
            description: description.into(),
        }
    }
}
