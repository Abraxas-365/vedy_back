use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct Tenant {
    pub id: i32,
    pub auth_user_id: String,
    pub company_name: Option<String>,
    pub first_name: String,
    pub last_name: String,
    pub phone: Option<String>,
}
