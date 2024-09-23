use async_trait::async_trait;
use sqlx::Row;

use crate::error::ApiError;
use crate::modules::front::landing::feedback::port::DBRepository;
use crate::modules::front::landing::feedback::Feedback;
use crate::utils::database::{Filter, PaginatedRecord, Pagination, PostgresRepository, Value};

#[async_trait]
impl DBRepository for PostgresRepository {
    async fn create(&self, feedback: Feedback) -> Result<Feedback, ApiError> {
        let created_feedback = sqlx::query(
            r#"
            INSERT INTO feedback (tenant_id, property_image, customer_image, customer_name, customer_review, description)
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING *
            "#,
        )
        .bind(&feedback.tenant_id)
        .bind(&feedback.property_image)
        .bind(&feedback.customer_image)
        .bind(&feedback.customer_name)
        .bind(&feedback.customer_review)
        .bind(&feedback.description)
        .fetch_one(&*self.pg_pool)
        .await
        .map_err(ApiError::DatabaseError)?;

        Ok(Feedback {
            id: created_feedback.get("id"),
            tenant_id: created_feedback.get("tenant_id"),
            property_image: created_feedback.get("property_image"),
            customer_image: created_feedback.get("customer_image"),
            customer_name: created_feedback.get("customer_name"),
            customer_review: created_feedback.get("customer_review"),
            description: created_feedback.get("description"),
        })
    }

    async fn edit(&self, feedback: Feedback) -> Result<Feedback, ApiError> {
        let updated_feedback = sqlx::query(
            r#"
            UPDATE feedback
            SET property_image = $1, customer_image = $2, customer_name = $3, customer_review = $4, description = $5
            WHERE id = $6 AND tenant_id = $7
            RETURNING *
            "#,
        )
        .bind(&feedback.property_image)
        .bind(&feedback.customer_image)
        .bind(&feedback.customer_name)
        .bind(&feedback.customer_review)
        .bind(&feedback.description)
        .bind(&feedback.id)
        .bind(&feedback.tenant_id)
        .fetch_one(&*self.pg_pool)
        .await
        .map_err(|err| match err {
            sqlx::Error::RowNotFound => ApiError::NotFound(format!("Feedback with id {} not found", feedback.id)),
            _ => ApiError::DatabaseError(err.into()),
        })?;

        Ok(Feedback {
            id: updated_feedback.get("id"),
            tenant_id: updated_feedback.get("tenant_id"),
            property_image: updated_feedback.get("property_image"),
            customer_image: updated_feedback.get("customer_image"),
            customer_name: updated_feedback.get("customer_name"),
            customer_review: updated_feedback.get("customer_review"),
            description: updated_feedback.get("description"),
        })
    }

    async fn find_many(
        &self,
        filter: Filter,
        pagination: Pagination,
    ) -> Result<PaginatedRecord<Feedback>, ApiError> {
        let (where_clause, args) = filter.build_for_sqlx();

        // Calculate offset
        let offset = (pagination.page - 1) * pagination.per_page;

        // Count total items
        let count_query = format!("SELECT COUNT(*) FROM feedback WHERE {}", where_clause);
        let mut count_query_builder = sqlx::query(&count_query);
        for arg in args.clone() {
            count_query_builder = match arg {
                Value::Int(i) => count_query_builder.bind(i),
                Value::Float(f) => count_query_builder.bind(f),
                Value::String(s) => count_query_builder.bind(s),
                Value::Bool(b) => count_query_builder.bind(b),
                Value::Json(j) => count_query_builder.bind(j),
            };
        }
        let total_items: i64 = count_query_builder
            .fetch_one(&*self.pg_pool)
            .await
            .map_err(ApiError::DatabaseError)?
            .get(0);

        // Fetch paginated items
        let query = format!(
            "SELECT * FROM feedback WHERE {} LIMIT {} OFFSET {}",
            where_clause, pagination.per_page, offset
        );
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

        let feedbacks = query_builder
            .fetch_all(&*self.pg_pool)
            .await
            .map_err(ApiError::DatabaseError)?;

        let feedbacks = feedbacks
            .into_iter()
            .map(|row| Feedback {
                id: row.get("id"),
                tenant_id: row.get("tenant_id"),
                property_image: row.get("property_image"),
                customer_image: row.get("customer_image"),
                customer_name: row.get("customer_name"),
                customer_review: row.get("customer_review"),
                description: row.get("description"),
            })
            .collect();

        Ok(PaginatedRecord::new(
            feedbacks,
            total_items as u64,
            pagination.page,
            pagination.per_page,
        ))
    }

    async fn delete(&self, id: i32, tenant_id: i32) -> Result<Feedback, ApiError> {
        let mut tx = self
            .pg_pool
            .begin()
            .await
            .map_err(ApiError::DatabaseError)?;

        // Fetch the feedback to return it after deletion
        let feedback_row = sqlx::query(
            r#"
            SELECT * FROM feedback
            WHERE id = $1 AND tenant_id = $2
            "#,
        )
        .bind(id)
        .bind(tenant_id)
        .fetch_one(&mut *tx)
        .await
        .map_err(|err| match err {
            sqlx::Error::RowNotFound => {
                ApiError::NotFound(format!("Feedback with id {} not found", id))
            }
            _ => ApiError::DatabaseError(err.into()),
        })?;

        let feedback = Feedback {
            id: feedback_row.get("id"),
            tenant_id: feedback_row.get("tenant_id"),
            property_image: feedback_row.get("property_image"),
            customer_image: feedback_row.get("customer_image"),
            customer_name: feedback_row.get("customer_name"),
            customer_review: feedback_row.get("customer_review"),
            description: feedback_row.get("description"),
        };

        // Delete feedback
        let result = sqlx::query("DELETE FROM feedback WHERE id = $1 AND tenant_id = $2")
            .bind(id)
            .bind(tenant_id)
            .execute(&mut *tx)
            .await
            .map_err(ApiError::DatabaseError)?;

        if result.rows_affected() == 0 {
            return Err(ApiError::NotFound(
                "Feedback not found or tenant mismatch".to_string(),
            ));
        }

        tx.commit().await.map_err(ApiError::DatabaseError)?;

        Ok(feedback)
    }
}
