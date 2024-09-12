use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct UserSession {
    pub(crate) id: String,
    pub(crate) expires_at: DateTime<Utc>,
    pub user_id: String,
}

impl UserSession {
    pub fn is_expired(&self) -> bool {
        self.expires_at < Utc::now()
    }
}
