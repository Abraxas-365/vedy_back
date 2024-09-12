use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

#[derive(Debug, Clone, Deserialize, Serialize, FromRow)]
pub struct Config {
    pub id: i32,
    pub tenant_id: i32,
    pub logo: String,
    pub color: String,
}
impl Config {
    pub fn new(tenant_id: i32, logo: &str, color: &str) -> Self {
        Self {
            id: 0,
            tenant_id,
            logo: logo.into(),
            color: color.into(),
        }
    }
}
