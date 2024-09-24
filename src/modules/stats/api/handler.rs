use actix_web::{web, HttpRequest, HttpResponse};
use serde::Deserialize;
use std::sync::Arc;

use crate::{
    error::ApiError,
    modules::stats::{EventType, Service, Stats},
    utils::lucia,
};

#[derive(Deserialize)]
pub struct CreateStats {
    pub event_type: EventType,
    pub tenant_id: i32,
    pub details: Option<serde_json::Value>,
}

pub async fn create_stats(
    service: web::Data<Arc<Service>>,
    lucia_service: web::Data<Arc<lucia::Service>>,
    req: web::Json<CreateStats>,
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
    let _session = lucia_service
        .get_session(basic_auth_header.unwrap())
        .await?;

    let stats = Stats {
        id: 0, // This will be set by the database
        event_type: req.event_type.clone(),
        tenant_id: req.tenant_id,
        details: req.details.clone(),
        created_at: chrono::Utc::now(),
    };

    let created_stats = service.create(stats).await?;
    Ok(HttpResponse::Created().json(created_stats))
}

pub async fn get_property_visited_info(
    service: web::Data<Arc<Service>>,
    tenant_id: web::Path<i32>,
) -> Result<HttpResponse, ApiError> {
    let property_visited_info = service.get_property_visited_info(*tenant_id).await?;
    Ok(HttpResponse::Ok().json(property_visited_info))
}

pub async fn get_landing_visited_info(
    service: web::Data<Arc<Service>>,
    tenant_id: web::Path<i32>,
) -> Result<HttpResponse, ApiError> {
    let landing_visited_info = service.get_landing_visited_info(*tenant_id).await?;
    Ok(HttpResponse::Ok().json(landing_visited_info))
}
