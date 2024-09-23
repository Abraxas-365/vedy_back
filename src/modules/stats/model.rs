use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, sqlx::Type)]
#[serde(rename_all = "snake_case")]
#[sqlx(type_name = "event_type")]
pub enum EventType {
    PropertyVisited,
    LandingVisited,
}

pub struct PropertyVisited {
    pub property_id: i32,
    pub metadata: Metadata,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Metadata {
    pub referrer: Option<String>,
    pub device_type: Option<String>, // e.g., "mobile", "desktop"
    pub ip_address: Option<String>,  // IP address of the visitor
    pub user_agent: Option<String>,  // User agent string of the browser
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LandingVisited {
    pub metadata: Metadata,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Stats {
    pub id: i32,
    pub event_type: EventType,
    pub tenant_id: i32,
    pub details: Option<serde_json::Value>,
    pub created_at: DateTime<Utc>,
}
