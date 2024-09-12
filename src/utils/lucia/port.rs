use async_trait::async_trait;

use super::{error::Error, UserSession};

#[async_trait]
pub trait Repository: Send + Sync {
    async fn get_session(&self, session_id: &str) -> Result<UserSession, Error>;
}
