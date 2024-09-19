use actix_web::{web, HttpRequest, HttpResponse, Responder};
use serde::Deserialize;
use std::sync::Arc;

use crate::{error::ApiError, modules::tenant::Service, utils::lucia};

#[derive(Debug, Deserialize)]
pub struct UpdateTenantRequest {
    pub company_name: Option<String>,
    pub first_name: String,
    pub last_name: String,
    pub phone: Option<String>,
}

pub async fn update_tenant(
    service: web::Data<Arc<Service>>,
    lucia_service: web::Data<Arc<lucia::Service>>,
    req_headers: HttpRequest,
    web::Json(update_request): web::Json<UpdateTenantRequest>,
) -> Result<impl Responder, ApiError> {
    // Extract the Authorization header
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

    let mut tenant = service.find_by_user_id(&session.user_id).await?;

    tenant.company_name = update_request.company_name;
    tenant.first_name = update_request.first_name;
    tenant.last_name = update_request.last_name;
    tenant.phone = update_request.phone;

    let updated_tenant = service.update_tenant(tenant).await?;

    Ok(HttpResponse::Ok().json(updated_tenant))
}

pub async fn get_tenant_by_id(
    service: web::Data<Arc<Service>>,
    lucia_service: web::Data<Arc<lucia::Service>>,
    req_headers: HttpRequest,
    web::Path(id): web::Path<String>,
) -> Result<impl Responder, ApiError> {
    // Extract the Authorization header
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

    // Ensure the session user_id matches the requested tenant id
    if session.user_id != id {
        return Err(ApiError::Unauthorized(
            "Unauthorized access to tenant data".into(),
        ));
    }

    let tenant = service.find_by_user_id(&id).await?;

    Ok(HttpResponse::Ok().json(tenant))
}
