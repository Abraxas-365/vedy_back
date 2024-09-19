use async_trait::async_trait;
use sqlx::Row;

use crate::error::ApiError;
use crate::modules::front::landing::hero::port::DBRepository;
use crate::modules::front::landing::hero::Hero;
use crate::utils::database::{Filter, PostgresRepository, Value};

#[async_trait]
impl DBRepository for PostgresRepository {
    async fn edit(&self, hero: Hero) -> Result<Hero, ApiError> {
        let updated_hero = sqlx::query(
            r#"
            UPDATE hero
            SET title = $1, description = $2, image = $3
            WHERE tenant_id = $4
            RETURNING *
            "#,
        )
        .bind(&hero.title)
        .bind(&hero.description)
        .bind(&hero.image)
        .bind(&hero.tenant_id)
        .fetch_one(&*self.pg_pool)
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => {
                ApiError::NotFound(format!("Hero for tenant_id {} not found", hero.tenant_id))
            }
            _ => ApiError::DatabaseError(e),
        })?;

        Ok(Hero {
            id: updated_hero.get("id"),
            tenant_id: updated_hero.get("tenant_id"),
            title: updated_hero.get("title"),
            description: updated_hero.get("description"),
            image: updated_hero.get("image"),
        })
    }

    async fn find(&self, filter: Filter) -> Result<Hero, ApiError> {
        let (where_clause, args) = filter.build_for_sqlx();

        let query = format!("SELECT * FROM hero WHERE {} LIMIT 1", where_clause);

        let mut query_builder = sqlx::query(&query);

        for arg in args {
            query_builder = match arg {
                Value::Int(i) => query_builder.bind(i),
                Value::Float(f) => query_builder.bind(f),
                Value::String(s) => query_builder.bind(s),
                Value::Bool(b) => query_builder.bind(b),
            };
        }

        let hero = query_builder
            .fetch_one(&*self.pg_pool)
            .await
            .map_err(|e| match e {
                sqlx::Error::RowNotFound => ApiError::NotFound("Hero not found".to_string()),
                _ => ApiError::DatabaseError(e),
            })?;

        Ok(Hero {
            id: hero.get("id"),
            tenant_id: hero.get("tenant_id"),
            title: hero.get("title"),
            description: hero.get("description"),
            image: hero.get("image"),
        })
    }
}
