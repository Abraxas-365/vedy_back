use async_trait::async_trait;
use sqlx::Row;

use crate::error::ApiError;
use crate::modules::hero::port::DBRepository;
use crate::modules::hero::Hero;
use crate::utils::database::{Filter, PostgresRepository};

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
        .map_err(ApiError::DatabaseError)?;

        Ok(Hero {
            id: updated_hero.get("id"),
            tenant_id: updated_hero.get("tenant_id"),
            title: updated_hero.get("title"),
            description: updated_hero.get("description"),
            image: updated_hero.get("image"),
            created_at: updated_hero.get("created_at"),
            updated_at: updated_hero.get("updated_at"),
        })
    }

    async fn find(&self, filter: Filter) -> Result<Hero, ApiError> {
        let (where_clause, _args) = filter.build_for_sqlx();

        let query = format!("SELECT * FROM hero WHERE {} LIMIT 1", where_clause);

        let hero = sqlx::query(&query)
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
            created_at: hero.get("created_at"),
            updated_at: hero.get("updated_at"),
        })
    }
}
