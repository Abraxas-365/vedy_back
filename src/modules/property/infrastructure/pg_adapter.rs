use async_trait::async_trait;
use sqlx::Row;

use crate::error::ApiError;
use crate::modules::property::port::DBRepository;
use crate::modules::property::{Property, PropertyImage, PropertyWithImages};
use crate::utils::database::{Filter, PaginatedRecord, Pagination, PostgresRepository};

#[async_trait]
impl DBRepository for PostgresRepository {
    async fn create(
        &self,
        property: Property,
        images: Vec<PropertyImage>,
    ) -> Result<PropertyWithImages, ApiError> {
        let mut tx = self
            .pg_pool
            .begin()
            .await
            .map_err(ApiError::DatabaseError)?;

        // Insert property
        let inserted_property = sqlx::query(
            r#"
            INSERT INTO properties (
                tenant_id, title, description, property_type, status, price, currency,
                bedrooms, bathrooms, parking_spaces, total_area, built_area, year_built,
                address, city, state, country, google_maps_url
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18)
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
        .fetch_one(&mut *tx)
        .await
        .map_err(ApiError::DatabaseError)?;

        let inserted_property = Property {
            id: inserted_property.get("id"),
            tenant_id: inserted_property.get("tenant_id"),
            title: inserted_property.get("title"),
            description: inserted_property.get("description"),
            property_type: inserted_property.get("property_type"),
            status: inserted_property.get("status"),
            price: inserted_property.get("price"),
            currency: inserted_property.get("currency"),
            bedrooms: inserted_property.get("bedrooms"),
            bathrooms: inserted_property.get("bathrooms"),
            parking_spaces: inserted_property.get("parking_spaces"),
            total_area: inserted_property.get("total_area"),
            built_area: inserted_property.get("built_area"),
            year_built: inserted_property.get("year_built"),
            address: inserted_property.get("address"),
            city: inserted_property.get("city"),
            state: inserted_property.get("state"),
            country: inserted_property.get("country"),
            google_maps_url: inserted_property.get("google_maps_url"),
            created_at: inserted_property.get("created_at"),
            updated_at: inserted_property.get("updated_at"),
        };

        // Insert images
        let mut inserted_images = Vec::new();
        for image in images {
            let inserted_image = sqlx::query(
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

            inserted_images.push(PropertyImage {
                id: inserted_image.get("id"),
                property_id: inserted_image.get("property_id"),
                image_url: inserted_image.get("image_url"),
                is_primary: inserted_image.get("is_primary"),
                created_at: inserted_image.get("created_at"),
                updated_at: inserted_image.get("updated_at"),
            });
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
        images: Vec<PropertyImage>,
    ) -> Result<Vec<PropertyImage>, ApiError> {
        let mut tx = self
            .pg_pool
            .begin()
            .await
            .map_err(ApiError::DatabaseError)?;

        // Delete existing images
        sqlx::query("DELETE FROM property_images WHERE property_id = $1")
            .bind(&property_id)
            .execute(&mut *tx)
            .await
            .map_err(ApiError::DatabaseError)?;

        // Insert new images
        let mut inserted_images = Vec::new();
        for image in images {
            let inserted_image = sqlx::query(
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

            inserted_images.push(PropertyImage {
                id: inserted_image.get("id"),
                property_id: inserted_image.get("property_id"),
                image_url: inserted_image.get("image_url"),
                is_primary: inserted_image.get("is_primary"),
                created_at: inserted_image.get("created_at"),
                updated_at: inserted_image.get("updated_at"),
            });
        }

        tx.commit().await.map_err(ApiError::DatabaseError)?;

        Ok(inserted_images)
    }

    async fn udate_property(
        &self,
        id: i32,
        property: Property,
    ) -> Result<PropertyWithImages, ApiError> {
        let mut tx = self
            .pg_pool
            .begin()
            .await
            .map_err(ApiError::DatabaseError)?;

        // Update property
        let updated_property = sqlx::query(
            r#"
            UPDATE properties
            SET tenant_id = $1, title = $2, description = $3, property_type = $4,
                status = $5, price = $6, currency = $7, bedrooms = $8, bathrooms = $9,
                parking_spaces = $10, total_area = $11, built_area = $12, year_built = $13,
                address = $14, city = $15, state = $16, country = $17, google_maps_url = $18,
                updated_at = CURRENT_TIMESTAMP
            WHERE id = $19
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
        .bind(&id)
        .fetch_one(&mut *tx)
        .await
        .map_err(ApiError::DatabaseError)?;

        let updated_property = Property {
            id: updated_property.get("id"),
            tenant_id: updated_property.get("tenant_id"),
            title: updated_property.get("title"),
            description: updated_property.get("description"),
            property_type: updated_property.get("property_type"),
            status: updated_property.get("status"),
            price: updated_property.get("price"),
            currency: updated_property.get("currency"),
            bedrooms: updated_property.get("bedrooms"),
            bathrooms: updated_property.get("bathrooms"),
            parking_spaces: updated_property.get("parking_spaces"),
            total_area: updated_property.get("total_area"),
            built_area: updated_property.get("built_area"),
            year_built: updated_property.get("year_built"),
            address: updated_property.get("address"),
            city: updated_property.get("city"),
            state: updated_property.get("state"),
            country: updated_property.get("country"),
            google_maps_url: updated_property.get("google_maps_url"),
            created_at: updated_property.get("created_at"),
            updated_at: updated_property.get("updated_at"),
        };

        // Fetch images
        let images = sqlx::query("SELECT * FROM property_images WHERE property_id = $1")
            .bind(&id)
            .fetch_all(&mut *tx)
            .await
            .map_err(ApiError::DatabaseError)?;

        let images = images
            .into_iter()
            .map(|row| PropertyImage {
                id: row.get("id"),
                property_id: row.get("property_id"),
                image_url: row.get("image_url"),
                is_primary: row.get("is_primary"),
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
            })
            .collect();

        tx.commit().await.map_err(ApiError::DatabaseError)?;

        Ok(PropertyWithImages {
            property: updated_property,
            images,
        })
    }

    async fn find(&self, filter: Filter) -> Result<PropertyWithImages, ApiError> {
        let (where_clause, _args) = filter.build_for_sqlx();

        let query = format!(
            "SELECT p.*, pi.id as image_id, pi.image_url, pi.is_primary
             FROM properties p
             LEFT JOIN property_images pi ON p.id = pi.property_id
             WHERE {}
             LIMIT 1",
            where_clause
        );

        let rows = sqlx::query(&query)
            .fetch_all(&*self.pg_pool)
            .await
            .map_err(ApiError::DatabaseError)?;

        if rows.is_empty() {
            return Err(ApiError::NotFound("Property not found".to_string()));
        }

        let property = Property {
            id: rows[0].get("id"),
            tenant_id: rows[0].get("tenant_id"),
            title: rows[0].get("title"),
            description: rows[0].get("description"),
            property_type: rows[0].get("property_type"),
            status: rows[0].get("status"),
            price: rows[0].get("price"),
            currency: rows[0].get("currency"),
            bedrooms: rows[0].get("bedrooms"),
            bathrooms: rows[0].get("bathrooms"),
            parking_spaces: rows[0].get("parking_spaces"),
            total_area: rows[0].get("total_area"),
            built_area: rows[0].get("built_area"),
            year_built: rows[0].get("year_built"),
            address: rows[0].get("address"),
            city: rows[0].get("city"),
            state: rows[0].get("state"),
            country: rows[0].get("country"),
            google_maps_url: rows[0].get("google_maps_url"),
            created_at: rows[0].get("created_at"),
            updated_at: rows[0].get("updated_at"),
        };

        let images = rows
            .into_iter()
            .filter_map(|row| {
                let image_id: Option<i32> = row.get("image_id");
                image_id.map(|_| PropertyImage {
                    id: row.get("image_id"),
                    property_id: row.get("id"),
                    image_url: row.get("image_url"),
                    is_primary: row.get("is_primary"),
                    created_at: row.get("created_at"),
                    updated_at: row.get("updated_at"),
                })
            })
            .collect();

        Ok(PropertyWithImages { property, images })
    }

    async fn find_many(
        &self,
        filter: Filter,
        pagination: Pagination,
    ) -> Result<PaginatedRecord<PropertyWithImages>, ApiError> {
        let (where_clause, _args) = filter.build_for_sqlx();

        let offset = (pagination.page - 1) * pagination.per_page;

        let query = format!(
            "SELECT p.*, pi.id as image_id, pi.image_url, pi.is_primary
             FROM properties p
             LEFT JOIN property_images pi ON p.id = pi.property_id
             WHERE {}
             ORDER BY p.id
             LIMIT {} OFFSET {}",
            where_clause, pagination.per_page, offset
        );

        let rows = sqlx::query(&query)
            .fetch_all(&*self.pg_pool)
            .await
            .map_err(ApiError::DatabaseError)?;

        let mut properties_with_images = Vec::new();
        let mut current_property: Option<Property> = None;
        let mut current_images = Vec::new();

        for row in rows {
            let property = Property {
                id: row.get("id"),
                tenant_id: row.get("tenant_id"),
                title: row.get("title"),
                description: row.get("description"),
                property_type: row.get("property_type"),
                status: row.get("status"),
                price: row.get("price"),
                currency: row.get("currency"),
                bedrooms: row.get("bedrooms"),
                bathrooms: row.get("bathrooms"),
                parking_spaces: row.get("parking_spaces"),
                total_area: row.get("total_area"),
                built_area: row.get("built_area"),
                year_built: row.get("year_built"),
                address: row.get("address"),
                city: row.get("city"),
                state: row.get("state"),
                country: row.get("country"),
                google_maps_url: row.get("google_maps_url"),
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
            };

            let image: Option<PropertyImage> =
                row.get::<Option<i32>, _>("image_id")
                    .map(|_| PropertyImage {
                        id: row.get("image_id"),
                        property_id: row.get("id"),
                        image_url: row.get("image_url"),
                        is_primary: row.get("is_primary"),
                        created_at: row.get("created_at"),
                        updated_at: row.get("updated_at"),
                    });

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
        let total_items: i64 = sqlx::query_scalar(&count_query)
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
}
