use actix_web::{web, HttpRequest, HttpResponse};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::{
    error::ApiError,
    modules::{
        front::landing::config::{Config, Service},
        tenant,
    },
    utils::lucia,
};

#[derive(Deserialize)]
pub struct UpdateConfig {
    pub logo: String,
    pub color: String,
}

#[derive(Serialize)]
pub struct PresignedUrlResponse {
    pub url: String,
}

pub async fn update_config(
    service: web::Data<Arc<Service>>,
    tenant_service: web::Data<Arc<tenant::Service>>,
    lucia_service: web::Data<Arc<lucia::Service>>,
    req: web::Json<UpdateConfig>,
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

    let config = Config::new(tenant.id, &req.logo, &req.color);

    let updated_config = service.update_config(config).await?;

    Ok(HttpResponse::Ok().json(updated_config))
}

pub async fn get_tenant_config(
    service: web::Data<Arc<Service>>,
    tenant_id: web::Path<i32>,
) -> Result<HttpResponse, ApiError> {
    let config = service.find_tenant_config(*tenant_id).await?;
    Ok(HttpResponse::Ok().json(config))
}

pub async fn generate_logo_presigned_url(
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
