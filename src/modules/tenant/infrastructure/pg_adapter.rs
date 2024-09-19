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
            SELECT id, auth_user_id, company_name, first_name, last_name, phone
            FROM tenants
            WHERE auth_user_id = $1
            "#,
        )
        .bind(id)
        .fetch_one(&*self.pg_pool)
        .await
        .map_err(|err| match err {
            sqlx::Error::RowNotFound => {
                ApiError::NotFound(format!("Tenant with auth_user_id {} not found", id))
            }
            _ => ApiError::DatabaseError(err.into()),
        })?;

        Ok(tenant)
    }

    async fn update(&self, tenant: Tenant) -> Result<Tenant, ApiError> {
        let updated_tenant = sqlx::query_as::<_, Tenant>(
            r#"
            UPDATE tenants
            SET company_name = $1,
                first_name = $2,
                last_name = $3,
                phone = $4
            WHERE id = $5
            RETURNING id, auth_user_id, company_name, first_name, last_name, phone
            "#,
        )
        .bind(&tenant.company_name)
        .bind(&tenant.first_name)
        .bind(&tenant.last_name)
        .bind(&tenant.phone)
        .bind(&tenant.id)
        .fetch_one(&*self.pg_pool)
        .await
        .map_err(|err| match err {
            sqlx::Error::RowNotFound => {
                ApiError::NotFound(format!("Tenant with id {} not found", tenant.id))
            }
            _ => ApiError::DatabaseError(err.into()),
        })?;

        Ok(updated_tenant)
    }
}
