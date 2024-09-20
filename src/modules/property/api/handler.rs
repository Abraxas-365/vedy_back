use std::sync::Arc;

use actix_web::{web, HttpRequest, HttpResponse};
use serde::Deserialize;

use crate::{
    error::ApiError,
    modules::{
        property::{Property, Service},
        tenant,
    },
    utils::{database::Pagination, lucia},
};

#[derive(Deserialize)]
pub struct CreateProperty {
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
    pub amenities: Option<Vec<String>>,
    pub images_urls: Vec<String>,
}

#[derive(Deserialize)]
pub struct UpdateProperty {
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
    pub amenities: Option<Vec<String>>,
    pub images: Vec<String>,
}

pub async fn create_property(
    service: web::Data<Arc<Service>>,
    tenant_service: web::Data<Arc<tenant::Service>>,
    lucia_service: web::Data<Arc<lucia::Service>>,
    req: web::Json<CreateProperty>,
    req_headers: HttpRequest,
) -> Result<HttpResponse, ApiError> {
    let basic_auth_header = req_headers
        .headers()
        .get("Authorization")
        .and_then(|header_value| header_value.to_str().ok());

    if basic_auth_header.is_none() {
        return Err(ApiError::Unauthorized(
            "Missing Authorization header".into(),
        ));
    }
    let session = lucia_service
        .get_session(basic_auth_header.unwrap())
        .await?;

    let tenant = tenant_service.find_by_user_id(&session.user_id).await?;
    let property = Property::new(
        tenant.id,
        &req.title,
        req.description.as_deref(),
        &req.property_type,
        &req.status,
        req.price,
        &req.currency,
        req.bedrooms,
        req.bathrooms,
        req.parking_spaces,
        req.total_area,
        req.built_area,
        req.year_built,
        req.address.as_deref(),
        req.city.as_deref(),
        req.state.as_deref(),
        req.country.as_deref(),
        req.amenities.clone(),
        req.google_maps_url.as_deref(),
    );
    let property_with_images = service.create(property, &req.images_urls).await?;

    Ok(HttpResponse::Created().json(property_with_images))
}

pub async fn update_property(
    service: web::Data<Arc<Service>>,
    tenant_service: web::Data<Arc<tenant::Service>>,
    lucia_service: web::Data<Arc<lucia::Service>>,
    property_id: web::Path<i32>,
    req: web::Json<UpdateProperty>,
    req_headers: HttpRequest,
) -> Result<HttpResponse, ApiError> {
    let basic_auth_header = req_headers
        .headers()
        .get("Authorization")
        .and_then(|header_value| header_value.to_str().ok());

    if basic_auth_header.is_none() {
        return Err(ApiError::Unauthorized(
            "Missing Authorization header".into(),
        ));
    }
    let session = lucia_service
        .get_session(basic_auth_header.unwrap())
        .await?;

    let tenant = tenant_service.find_by_user_id(&session.user_id).await?;
    let property = Property {
        id: *property_id,
        tenant_id: tenant.id,
        title: req.title.clone(),
        description: req.description.clone(),
        property_type: req.property_type.clone(),
        status: req.status.clone(),
        price: req.price,
        currency: req.currency.clone(),
        bedrooms: req.bedrooms,
        bathrooms: req.bathrooms,
        parking_spaces: req.parking_spaces,
        total_area: req.total_area,
        built_area: req.built_area,
        year_built: req.year_built,
        address: req.address.clone(),
        city: req.city.clone(),
        state: req.state.clone(),
        country: req.country.clone(),
        google_maps_url: req.google_maps_url.clone(),
        amenities: req.amenities.clone(),
    };

    let updated_property = service.update_property(property, &req.images).await?;

    Ok(HttpResponse::Ok().json(updated_property))
}

pub async fn get_tenant_properties(
    service: web::Data<Arc<Service>>,
    tenant_id: web::Path<i32>,
    web::Query(pagination): web::Query<Pagination>,
) -> Result<HttpResponse, ApiError> {
    let properties = service
        .find_all_tenant_properties(*tenant_id, pagination.into())
        .await?;

    Ok(HttpResponse::Ok().json(properties))
}

pub async fn get_property_by_id(
    service: web::Data<Arc<Service>>,
    property_id: web::Path<i32>,
) -> Result<HttpResponse, ApiError> {
    let property = service.find_property_by_id(*property_id).await?;
    Ok(HttpResponse::Ok().json(property))
}

#[derive(Deserialize)]
pub struct GeneratePresignedUrls {
    pub n_links: usize,
}

pub async fn generate_presigned_urls(
    service: web::Data<Arc<Service>>,
    tenant_id: web::Path<i32>,
    req: web::Json<GeneratePresignedUrls>,
) -> Result<HttpResponse, ApiError> {
    let urls = service
        .generate_post_presigned_urls(*tenant_id, req.n_links)
        .await?;
    Ok(HttpResponse::Ok().json(urls))
}

pub async fn delete_property(
    service: web::Data<Arc<Service>>,
    lucia_service: web::Data<Arc<lucia::Service>>,
    tenant_service: web::Data<Arc<tenant::Service>>,
    property_id: web::Path<i32>,
    req_headers: HttpRequest,
) -> Result<HttpResponse, ApiError> {
    let basic_auth_header = req_headers
        .headers()
        .get("Authorization")
        .and_then(|header_value| header_value.to_str().ok());

    if basic_auth_header.is_none() {
        return Err(ApiError::Unauthorized(
            "Missing Authorization header".into(),
        ));
    }
    let session = lucia_service
        .get_session(basic_auth_header.unwrap())
        .await?;

    let tenant = tenant_service.find_by_user_id(&session.user_id).await?;
    let deleted_property = service.delete_property(*property_id, tenant.id).await?;
    Ok(HttpResponse::Ok().json(deleted_property))
}
