use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Deserialize, Serialize, FromRow)]
pub struct Property {
    pub id: i32,
    pub tenant_id: i32,
    pub title: String,
    pub description: Option<String>,
    pub property_type: String,
    pub status: String,
    pub price: f64,
    pub currency: String,
    pub bedrooms: Option<i32>,
    pub bathrooms: Option<i32>,
    pub parking_spaces: Option<i32>,
    pub total_area: Option<f64>,
    pub built_area: Option<f64>,
    pub year_built: Option<i32>,
    pub address: Option<String>,
    pub city: Option<String>,
    pub state: Option<String>,
    pub country: Option<String>,
    pub google_maps_url: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Property {
    pub fn new<S, T>(
        tenant_id: i32,
        title: S,
        description: Option<T>,
        property_type: S,
        status: S,
        price: f64,
        currency: S,
        bedrooms: Option<i32>,
        bathrooms: Option<i32>,
        parking_spaces: Option<i32>,
        total_area: Option<f64>,
        built_area: Option<f64>,
        year_built: Option<i32>,
        address: Option<T>,
        city: Option<T>,
        state: Option<T>,
        country: Option<T>,
        google_maps_url: Option<T>,
    ) -> Self
    where
        S: AsRef<str>,
        T: AsRef<str>,
    {
        Self {
            id: 0,
            tenant_id,
            title: title.as_ref().to_string(),
            description: description.map(|s| s.as_ref().to_string()),
            property_type: property_type.as_ref().to_string(),
            status: status.as_ref().to_string(),
            price,
            currency: currency.as_ref().to_string(),
            bedrooms,
            bathrooms,
            parking_spaces,
            total_area,
            built_area,
            year_built,
            address: address.map(|s| s.as_ref().to_string()),
            city: city.map(|s| s.as_ref().to_string()),
            state: state.map(|s| s.as_ref().to_string()),
            country: country.map(|s| s.as_ref().to_string()),
            google_maps_url: google_maps_url.map(|s| s.as_ref().to_string()),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, FromRow)]
pub struct PropertyImage {
    pub id: i32,
    pub property_id: i32,
    pub image_url: String,
    pub is_primary: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl PropertyImage {
    pub fn new(property_id: i32, image_url: &str, is_primary: bool) -> Self {
        Self {
            id: 0,
            property_id,
            image_url: image_url.into(),
            is_primary,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PropertyWithImages {
    pub property: Property,
    pub images: Vec<PropertyImage>,
}
