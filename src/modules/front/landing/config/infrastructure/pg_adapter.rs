use async_trait::async_trait;
use sqlx::Row;

use crate::error::ApiError;
use crate::modules::front::landing::config::port::DBRepository;
use crate::modules::front::landing::config::Config;
use crate::utils::database::{Filter, PostgresRepository, Value};

#[async_trait]
impl DBRepository for PostgresRepository {
    async fn edit(&self, config: Config) -> Result<Config, ApiError> {
        let updated_config = sqlx::query(
            r#"
            UPDATE config
            SET logo = $1, color = $2
            WHERE tenant_id = $3
            RETURNING *
            "#,
        )
        .bind(&config.logo)
        .bind(&config.color)
        .bind(&config.tenant_id)
        .fetch_one(&*self.pg_pool)
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => ApiError::NotFound(format!(
                "Config for tenant_id {} not found",
                config.tenant_id
            )),
            _ => ApiError::DatabaseError(e),
        })?;

        Ok(Config {
            id: updated_config.get("id"),
            tenant_id: updated_config.get("tenant_id"),
            logo: updated_config.get("logo"),
            color: updated_config.get("color"),
        })
    }

    async fn find(&self, filter: Filter) -> Result<Config, ApiError> {
        let (where_clause, args) = filter.build_for_sqlx();

        let query = format!("SELECT * FROM config WHERE {} LIMIT 1", where_clause);

        let mut query_builder = sqlx::query(&query);

        for arg in args {
            query_builder = match arg {
                Value::Int(i) => query_builder.bind(i),
                Value::Float(f) => query_builder.bind(f),
                Value::String(s) => query_builder.bind(s),
                Value::Bool(b) => query_builder.bind(b),
                Value::Json(j) => query_builder.bind(j),
            };
        }

        let config = query_builder
            .fetch_one(&*self.pg_pool)
            .await
            .map_err(|e| match e {
                sqlx::Error::RowNotFound => ApiError::NotFound("Config not found".to_string()),
                _ => ApiError::DatabaseError(e),
            })?;

        Ok(Config {
            id: config.get("id"),
            tenant_id: config.get("tenant_id"),
            logo: config.get("logo"),
            color: config.get("color"),
        })
    }
}
