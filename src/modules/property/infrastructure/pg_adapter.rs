use async_trait::async_trait;
use sqlx::FromRow;
use sqlx::Row;

use crate::error::ApiError;
use crate::modules::property::port::DBRepository;
use crate::modules::property::{Property, PropertyImage, PropertyWithImages};
use crate::utils::database::{Filter, PaginatedRecord, Pagination, PostgresRepository, Value};

#[async_trait]
impl DBRepository for PostgresRepository {
    async fn create(
        &self,
        property: Property,
        images: &[PropertyImage],
    ) -> Result<PropertyWithImages, ApiError> {
        let mut tx = self
            .pg_pool
            .begin()
            .await
            .map_err(ApiError::DatabaseError)?;

        // Insert property
        let inserted_property = sqlx::query_as::<_, Property>(
            r#"
            INSERT INTO properties (
                tenant_id, title, description, property_type, status, price, currency,
                bedrooms, bathrooms, parking_spaces, total_area, built_area, year_built,
                address, city, state, country, google_maps_url, amenities
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19)
            RETURNING *
            "#,
        )
        .bind(&property.tenant_id)
        .bind(&property.title)
        .bind(&property.description)
        .bind(&property.property_type)
        .bind(&property.status)
        .bind(&property.price)
        .bind(&property.currency)
        .bind(&property.bedrooms)
        .bind(&property.bathrooms)
        .bind(&property.parking_spaces)
        .bind(&property.total_area)
        .bind(&property.built_area)
        .bind(&property.year_built)
        .bind(&property.address)
        .bind(&property.city)
        .bind(&property.state)
        .bind(&property.country)
        .bind(&property.google_maps_url)
        .bind(&property.amenities)
        .fetch_one(&mut *tx)
        .await
        .map_err(ApiError::DatabaseError)?;

        // Insert images
        let mut inserted_images = Vec::new();
        for image in images {
            let inserted_image = sqlx::query_as::<_, PropertyImage>(
                r#"
                INSERT INTO property_images (property_id, image_url, is_primary)
                VALUES ($1, $2, $3)
                RETURNING *
                "#,
            )
            .bind(&inserted_property.id)
            .bind(&image.image_url)
            .bind(&image.is_primary)
            .fetch_one(&mut *tx)
            .await
            .map_err(ApiError::DatabaseError)?;

            inserted_images.push(inserted_image);
        }

        tx.commit().await.map_err(ApiError::DatabaseError)?;

        Ok(PropertyWithImages {
            property: inserted_property,
            images: inserted_images,
        })
    }

    async fn edit_property_images(
        &self,
        property_id: i32,
        images: &[PropertyImage],
    ) -> Result<Vec<PropertyImage>, ApiError> {
        let mut tx = self
            .pg_pool
            .begin()
            .await
            .map_err(ApiError::DatabaseError)?;

        // Check if the property exists
        let property_exists =
            sqlx::query_scalar::<_, bool>("SELECT EXISTS(SELECT 1 FROM properties WHERE id = $1)")
                .bind(property_id)
                .fetch_one(&mut *tx)
                .await
                .map_err(ApiError::DatabaseError)?;

        if !property_exists {
            return Err(ApiError::NotFound(format!(
                "Property with id {} not found",
                property_id
            )));
        }

        // Delete existing images
        sqlx::query("DELETE FROM property_images WHERE property_id = $1")
            .bind(&property_id)
            .execute(&mut *tx)
            .await
            .map_err(ApiError::DatabaseError)?;

        // Insert new images
        let mut inserted_images = Vec::new();
        for image in images {
            let inserted_image = sqlx::query_as::<_, PropertyImage>(
                r#"
                INSERT INTO property_images (property_id, image_url, is_primary)
                VALUES ($1, $2, $3)
                RETURNING *
                "#,
            )
            .bind(&property_id)
            .bind(&image.image_url)
            .bind(&image.is_primary)
            .fetch_one(&mut *tx)
            .await
            .map_err(ApiError::DatabaseError)?;

            inserted_images.push(inserted_image);
        }

        tx.commit().await.map_err(ApiError::DatabaseError)?;

        Ok(inserted_images)
    }

    async fn update_property(&self, property: Property) -> Result<PropertyWithImages, ApiError> {
        let mut tx = self
            .pg_pool
            .begin()
            .await
            .map_err(ApiError::DatabaseError)?;

        // Update property
        let updated_property = sqlx::query_as::<_, Property>(
            r#"
            UPDATE properties
            SET tenant_id = $1, title = $2, description = $3, property_type = $4,
                status = $5, price = $6, currency = $7, bedrooms = $8, bathrooms = $9,
                parking_spaces = $10, total_area = $11, built_area = $12, year_built = $13,
                address = $14, city = $15, state = $16, country = $17, google_maps_url = $18,
                amenities = $19, updated_at = CURRENT_TIMESTAMP
            WHERE id = $20
            RETURNING *
            "#,
        )
        .bind(&property.tenant_id)
        .bind(&property.title)
        .bind(&property.description)
        .bind(&property.property_type)
        .bind(&property.status)
        .bind(&property.price)
        .bind(&property.currency)
        .bind(&property.bedrooms)
        .bind(&property.bathrooms)
        .bind(&property.parking_spaces)
        .bind(&property.total_area)
        .bind(&property.built_area)
        .bind(&property.year_built)
        .bind(&property.address)
        .bind(&property.city)
        .bind(&property.state)
        .bind(&property.country)
        .bind(&property.google_maps_url)
        .bind(&property.amenities)
        .bind(&property.id)
        .fetch_one(&mut *tx)
        .await
        .map_err(|err| match err {
            sqlx::Error::RowNotFound => {
                ApiError::NotFound(format!("Property with id {} not found", property.id))
            }
            _ => ApiError::DatabaseError(err),
        })?;

        // Fetch images
        let images = sqlx::query_as::<_, PropertyImage>(
            "SELECT * FROM property_images WHERE property_id = $1",
        )
        .bind(&property.id)
        .fetch_all(&mut *tx)
        .await
        .map_err(ApiError::DatabaseError)?;

        tx.commit().await.map_err(ApiError::DatabaseError)?;

        Ok(PropertyWithImages {
            property: updated_property,
            images,
        })
    }

    async fn find(&self, filter: Filter) -> Result<PropertyWithImages, ApiError> {
        let (where_clause, args) = filter.build_for_sqlx();

        let query = format!(
            "SELECT p.*, pi.id as image_id, pi.image_url, pi.is_primary
             FROM properties p
             LEFT JOIN property_images pi ON p.id = pi.property_id
             WHERE {}
             LIMIT 1",
            where_clause.replace("id", "p.id")
        );

        let mut query_builder = sqlx::query(&query);

        for arg in args {
            query_builder = match arg {
                Value::Int(i) => query_builder.bind(i),
                Value::Float(f) => query_builder.bind(f),
                Value::String(s) => query_builder.bind(s),
                Value::Bool(b) => query_builder.bind(b),
            };
        }

        let rows = query_builder
            .fetch_all(&*self.pg_pool)
            .await
            .map_err(ApiError::DatabaseError)?;

        if rows.is_empty() {
            return Err(ApiError::NotFound("Property not found".to_string()));
        }

        let property: Property =
            Property::from_row(&rows[0]).map_err(|e| ApiError::DatabaseError(e.into()))?;

        let images: Vec<PropertyImage> = rows
            .into_iter()
            .filter_map(|row| {
                let image_id: Option<i32> = row.get("image_id");
                image_id
                    .map(|_| PropertyImage::from_row(&row).ok())
                    .flatten()
            })
            .collect();

        Ok(PropertyWithImages { property, images })
    }

    async fn find_many(
        &self,
        filter: Filter,
        pagination: Pagination,
    ) -> Result<PaginatedRecord<PropertyWithImages>, ApiError> {
        let (where_clause, mut args) = filter.build_for_sqlx();

        let offset = (pagination.page - 1) * pagination.per_page;

        let query = format!(
            "SELECT p.*, pi.id as image_id, pi.image_url, pi.is_primary
         FROM properties p
         LEFT JOIN property_images pi ON p.id = pi.property_id
         WHERE {}
         ORDER BY p.id
         LIMIT ${} OFFSET ${}",
            where_clause,
            args.len() + 1,
            args.len() + 2
        );

        // Add LIMIT and OFFSET to args
        args.push(Value::Int(pagination.per_page as i64));
        args.push(Value::Int(offset as i64));

        let mut query_builder = sqlx::query(&query);

        for arg in args.iter() {
            query_builder = match arg {
                Value::Int(i) => query_builder.bind(i),
                Value::Float(f) => query_builder.bind(f),
                Value::String(s) => query_builder.bind(s),
                Value::Bool(b) => query_builder.bind(b),
            };
        }

        let rows = query_builder
            .fetch_all(&*self.pg_pool)
            .await
            .map_err(ApiError::DatabaseError)?;

        let mut properties_with_images = Vec::new();
        let mut current_property: Option<Property> = None;
        let mut current_images = Vec::new();

        for row in rows {
            let property =
                Property::from_row(&row).map_err(|e| ApiError::DatabaseError(e.into()))?;

            let image: Option<PropertyImage> = row
                .get::<Option<i32>, _>("image_id")
                .map(|_| PropertyImage::from_row(&row).ok())
                .flatten();

            if let Some(current) = &current_property {
                if current.id != property.id {
                    properties_with_images.push(PropertyWithImages {
                        property: current.clone(),
                        images: std::mem::take(&mut current_images),
                    });
                    current_property = Some(property);
                }
            } else {
                current_property = Some(property);
            }

            if let Some(img) = image {
                current_images.push(img);
            }
        }

        if let Some(last_property) = current_property {
            properties_with_images.push(PropertyWithImages {
                property: last_property,
                images: current_images,
            });
        }

        // Count total items
        let count_query = format!(
            "SELECT COUNT(DISTINCT p.id) FROM properties p WHERE {}",
            where_clause
        );

        let mut count_query_builder = sqlx::query_scalar(&count_query);

        for arg in args.iter().take(args.len() - 2) {
            // Exclude LIMIT and OFFSET
            count_query_builder = match arg {
                Value::Int(i) => count_query_builder.bind(i),
                Value::Float(f) => count_query_builder.bind(f),
                Value::String(s) => count_query_builder.bind(s),
                Value::Bool(b) => count_query_builder.bind(b),
            };
        }

        let total_items: i64 = count_query_builder
            .fetch_one(&*self.pg_pool)
            .await
            .map_err(ApiError::DatabaseError)?;

        Ok(PaginatedRecord::new(
            properties_with_images,
            total_items as u64,
            pagination.page,
            pagination.per_page,
        ))
    }

    async fn delete(&self, id: i32, tenant_id: i32) -> Result<PropertyWithImages, ApiError> {
        let mut tx = self
            .pg_pool
            .begin()
            .await
            .map_err(ApiError::DatabaseError)?;

        // Fetch the property to return it after deletion
        let property_row = sqlx::query(
            r#"
            SELECT p.*, pi.id as image_id, pi.image_url, pi.is_primary
            FROM properties p
            LEFT JOIN property_images pi ON p.id = pi.property_id
            WHERE p.id = $1 AND p.tenant_id = $2
            "#,
        )
        .bind(id)
        .bind(tenant_id)
        .fetch_all(&mut *tx)
        .await
        .map_err(ApiError::DatabaseError)?;

        if property_row.is_empty() {
            return Err(ApiError::NotFound(
                "Property not found or tenant mismatch".to_string(),
            ));
        }

        let property =
            Property::from_row(&property_row[0]).map_err(|e| ApiError::DatabaseError(e.into()))?;

        let images: Vec<PropertyImage> = property_row
            .into_iter()
            .filter_map(|row| {
                let image_id: Option<i32> = row.get("image_id");
                image_id
                    .map(|_| PropertyImage::from_row(&row).ok())
                    .flatten()
            })
            .collect();

        // Delete property
        let result = sqlx::query("DELETE FROM properties WHERE id = $1 AND tenant_id = $2")
            .bind(id)
            .bind(tenant_id)
            .execute(&mut *tx)
            .await
            .map_err(ApiError::DatabaseError)?;

        if result.rows_affected() == 0 {
            return Err(ApiError::NotFound(
                "Property not found or tenant mismatch".to_string(),
            ));
        }

        tx.commit().await.map_err(ApiError::DatabaseError)?;

        Ok(PropertyWithImages { property, images })
    }
}
