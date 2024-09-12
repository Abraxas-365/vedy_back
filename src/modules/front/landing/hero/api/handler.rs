// handler.rs

use actix_web::{web, HttpRequest, HttpResponse};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::{
    error::ApiError,
    modules::{
        front::landing::hero::{Hero, Service},
        tenant,
    },
    utils::lucia,
};

#[derive(Deserialize)]
pub struct UpdateHero {
    pub title: String,
    pub description: String,
    pub image: String,
}

#[derive(Serialize)]
pub struct PresignedUrlResponse {
    pub url: String,
}

pub async fn update_hero(
    service: web::Data<Arc<Service>>,
    tenant_service: web::Data<Arc<tenant::Service>>,
    lucia_service: web::Data<Arc<lucia::Service>>,
    req: web::Json<UpdateHero>,
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

    let hero = Hero::new(tenant.id, &req.title, &req.description, &req.image);

    let updated_hero = service.update_hero(hero).await?;

    Ok(HttpResponse::Ok().json(updated_hero))
}

pub async fn get_tenant_hero(
    service: web::Data<Arc<Service>>,
    tenant_id: web::Path<i32>,
) -> Result<HttpResponse, ApiError> {
    let hero = service.find_tenant_hero(*tenant_id).await?;
    Ok(HttpResponse::Ok().json(hero))
}

pub async fn generate_hero_image_presigned_url(
    service: web::Data<Arc<Service>>,
    tenant_service: web::Data<Arc<tenant::Service>>,
    lucia_service: web::Data<Arc<lucia::Service>>,
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

    let url = service.generate_post_presigned_urls(tenant.id).await?;

    Ok(HttpResponse::Ok().json(PresignedUrlResponse { url }))
}
