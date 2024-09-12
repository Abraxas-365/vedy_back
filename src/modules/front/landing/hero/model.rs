use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

#[derive(Debug, Clone, Deserialize, Serialize, FromRow)]
pub struct Hero {
    pub id: i32,
    pub tenant_id: i32,
    pub title: String,
    pub description: String,
    pub image: String,
}
impl Hero {
    pub fn new(tenant_id: i32, title: &str, description: &str, image: &str) -> Self {
        Self {
            id: 0,
            tenant_id,
            title: title.into(),
            description: description.into(),
            image: image.into(),
        }
    }
}
