use crate::{
    error::ApiError,
    modules::front::social_media::{port::DBRepository, SocialMedia},
};
use async_trait::async_trait;
use sqlx::Row;

pub struct PostgresRepository {
    pub pg_pool: sqlx::PgPool,
}

#[async_trait]
impl DBRepository for PostgresRepository {
    async fn upsert(
        &self,
        social_media: SocialMedia,
        tenant_id: i32,
    ) -> Result<SocialMedia, ApiError> {
        let upserted_social_media = sqlx::query(
            r#"
            INSERT INTO social_media_links (tenant_id, facebook_url, instagram_url, tiktok_url, linkedin_url)
            VALUES ($1, $2, $3, $4, $5)
            ON CONFLICT (tenant_id) DO UPDATE
            SET facebook_url = EXCLUDED.facebook_url,
                instagram_url = EXCLUDED.instagram_url,
                tiktok_url = EXCLUDED.tiktok_url,
                linkedin_url = EXCLUDED.linkedin_url
            RETURNING *
            "#,
        )
        .bind(tenant_id)
        .bind(&social_media.facebook_url)
        .bind(&social_media.instagram_url)
        .bind(&social_media.tiktok_url)
        .bind(&social_media.linkedin_url)
        .fetch_one(&self.pg_pool)
        .await
        .map_err(ApiError::DatabaseError)?;

        Ok(SocialMedia {
            id: upserted_social_media.get("id"),
            tenant_id: upserted_social_media.get("tenant_id"),
            facebook_url: upserted_social_media.get("facebook_url"),
            instagram_url: upserted_social_media.get("instagram_url"),
            tiktok_url: upserted_social_media.get("tiktok_url"),
            linkedin_url: upserted_social_media.get("linkedin_url"),
        })
    }

    async fn find(&self, tenant_id: i32) -> Result<SocialMedia, ApiError> {
        let social_media = sqlx::query(
            r#"
            SELECT * FROM social_media_links
            WHERE tenant_id = $1
            "#,
        )
        .bind(tenant_id)
        .fetch_one(&self.pg_pool)
        .await
        .map_err(|err| match err {
            sqlx::Error::RowNotFound => ApiError::NotFound(format!(
                "Social media links for tenant_id {} not found",
                tenant_id
            )),
            _ => ApiError::DatabaseError(err.into()),
        })?;

        Ok(SocialMedia {
            id: social_media.get("id"),
            tenant_id: social_media.get("tenant_id"),
            facebook_url: social_media.get("facebook_url"),
            instagram_url: social_media.get("instagram_url"),
            tiktok_url: social_media.get("tiktok_url"),
            linkedin_url: social_media.get("linkedin_url"),
        })
    }
}
