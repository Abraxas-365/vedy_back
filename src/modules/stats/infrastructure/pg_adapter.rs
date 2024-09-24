use async_trait::async_trait;
use chrono::Utc;
use sqlx::Row;

use crate::{
    error::ApiError,
    modules::stats::{port::DBRepository, Info, LandingVisitedInfo, PropertyVisitedInfo, Stats},
    utils::database::PostgresRepository,
};

#[async_trait]
impl DBRepository for PostgresRepository {
    async fn create(&self, stats: Stats) -> Result<Stats, ApiError> {
        let row = sqlx::query(
            "INSERT INTO stats (event_type, tenant_id, details, created_at) 
             VALUES ($1, $2, $3, $4) 
             RETURNING id, event_type, tenant_id, details, created_at",
        )
        .bind(&stats.event_type)
        .bind(stats.tenant_id)
        .bind(&stats.details)
        .bind(Utc::now())
        .fetch_one(&*self.pg_pool)
        .await
        .map_err(|e| ApiError::DatabaseError(e))?;

        Ok(Stats {
            id: row.get("id"),
            event_type: row.get("event_type"),
            tenant_id: row.get("tenant_id"),
            details: row.get("details"),
            created_at: row.get("created_at"),
        })
    }

    async fn get_property_visited_info(
        &self,
        tenant_id: i32,
    ) -> Result<Vec<PropertyVisitedInfo>, ApiError> {
        let rows = sqlx::query(
            "SELECT 
                (s.details->>'property_id')::int as property_id,
                COUNT(*) as visit_count,
                COALESCE(ARRAY_AGG(DISTINCT s.details->'metadata'->>'referrer') FILTER (WHERE s.details->'metadata'->>'referrer' IS NOT NULL), ARRAY[]::TEXT[]) as referrer
             FROM stats s
             WHERE s.tenant_id = $1 
             AND s.event_type = 'property_visited'
             GROUP BY s.details->>'property_id'"
        )
        .bind(tenant_id)
        .fetch_all(&*self.pg_pool)
        .await
        .map_err(|e| ApiError::DatabaseError(e))?;

        if rows.is_empty() {
            return Err(ApiError::NotFound(format!(
                "No property visit data found for tenant_id: {}",
                tenant_id
            )));
        }

        let property_visited_info = rows
            .into_iter()
            .map(|row| PropertyVisitedInfo {
                property_id: row.get("property_id"),
                info: Info {
                    visit_count: row.get::<i64, _>("visit_count") as i32,
                    referrer: row.get::<Vec<String>, _>("referrer"),
                },
            })
            .collect();

        Ok(property_visited_info)
    }

    async fn get_landing_visited_info(
        &self,
        tenant_id: i32,
    ) -> Result<LandingVisitedInfo, ApiError> {
        let row = sqlx::query(
            "SELECT 
                COUNT(*) as visit_count,
                COALESCE(ARRAY_AGG(DISTINCT s.details->'metadata'->>'referrer') FILTER (WHERE s.details->'metadata'->>'referrer' IS NOT NULL), ARRAY[]::TEXT[]) as referrer
             FROM stats s
             WHERE s.tenant_id = $1 
             AND s.event_type = 'landing_visited'"
        )
        .bind(tenant_id)
        .fetch_optional(&*self.pg_pool)
        .await
        .map_err(|e| ApiError::DatabaseError(e))?;

        match row {
            Some(row) => Ok(LandingVisitedInfo {
                tenant_id,
                info: Info {
                    visit_count: row.get::<i64, _>("visit_count") as i32,
                    referrer: row.get::<Vec<String>, _>("referrer"),
                },
            }),
            None => Err(ApiError::NotFound(format!(
                "No landing visit data found for tenant_id: {}",
                tenant_id
            ))),
        }
    }
}
