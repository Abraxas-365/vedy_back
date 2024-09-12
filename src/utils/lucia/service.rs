use crate::utils::database::PostgresRepository;

use super::{error::Error, Repository, UserSession};
use std::sync::Arc;

#[derive(Clone)]
pub struct Service {
    repo: Arc<dyn Repository>,
}

impl Service {
    pub fn new(repo: Arc<dyn Repository>) -> Self {
        Self { repo }
    }

    pub async fn new_postgres() -> Self {
        let postgres_repo = PostgresRepository::new().await;

        Self::new(Arc::new(postgres_repo))
    }

    pub async fn get_session(&self, session_id: &str) -> Result<UserSession, Error> {
        let user_session = self.repo.get_session(session_id).await?;

        if user_session.is_expired() {
            return Err(Error::SessionExpired);
        }
        Ok(user_session)
    }
}
