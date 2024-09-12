use crate::utils::{
    database::PostgresRepository,
    lucia::{error::Error, Repository, UserSession},
};
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sqlx::Row;

#[async_trait]
impl Repository for PostgresRepository {
    async fn get_session(&self, session_id: &str) -> Result<UserSession, Error> {
        let query = sqlx::query("SELECT id, expires_at, user_id FROM user_session WHERE id = $1")
            .bind(session_id.to_string());

        let row = query.fetch_one(&*self.pg_pool).await.map_err(|e| match e {
            sqlx::Error::RowNotFound => Error::UserSessionNotFound,
            _ => Error::DatabaseQueryError(e.to_string()),
        })?;

        let user_session = UserSession {
            id: row
                .try_get("id")
                .map_err(|e| Error::DatabaseQueryError(e.to_string()))?,
            expires_at: row
                .try_get::<DateTime<Utc>, _>("expires_at")
                .map_err(|e| Error::DatabaseQueryError(e.to_string()))?,
            user_id: row
                .try_get("user_id")
                .map_err(|e| Error::DatabaseQueryError(e.to_string()))?,
        };

        if user_session.is_expired() {
            return Err(Error::SessionExpired);
        }

        Ok(user_session)
    }
}
