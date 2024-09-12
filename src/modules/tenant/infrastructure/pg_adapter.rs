use async_trait::async_trait;

use crate::{
    error::ApiError,
    modules::tenant::{port::DBRepository, Tenant},
    utils::database::PostgresRepository,
};

#[async_trait]
impl DBRepository for PostgresRepository {
    async fn find_by_user_id(&self, id: &str) -> Result<Tenant, ApiError> {
        let tenant = sqlx::query_as::<_, Tenant>(
            r#"
            SELECT id, auth_user_id, company_name, first_name, last_name, phone, created_at, updated_at
            FROM tenants
            WHERE auth_user_id = $1
            "#,
        )
        .bind(id)
        .fetch_one(&*self.pg_pool)
        .await
        .map_err(ApiError::DatabaseError)?;

        Ok(tenant)
    }
}
