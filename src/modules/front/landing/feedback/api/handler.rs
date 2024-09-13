// feedback/handler.rs
use actix_web::{web, HttpRequest, HttpResponse};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::{
    error::ApiError,
    modules::{
        front::landing::feedback::{Feedback, Service},
        tenant,
    },
    utils::{database::Pagination, lucia},
};

#[derive(Deserialize)]
pub struct CreateUpdateFeedback {
    pub property_image: String,
    pub customer_image: String,
    pub customer_name: String,
    pub customer_review: String,
    pub description: String,
}

#[derive(Serialize)]
pub struct PresignedUrlResponse {
    pub url: String,
}

pub async fn create_feedback(
    service: web::Data<Arc<Service>>,
    tenant_service: web::Data<Arc<tenant::Service>>,
    lucia_service: web::Data<Arc<lucia::Service>>,
    req: web::Json<CreateUpdateFeedback>,
    req_headers: HttpRequest,
) -> Result<HttpResponse, ApiError> {
    let basic_auth_header = req_headers
        .headers()
        .get("Authorization")
        .and_then(|header_value| header_value.to_str().ok())
        .ok_or_else(|| ApiError::Unauthorized("Missing Authorization header".into()))?;

    let session = lucia_service.get_session(basic_auth_header).await?;
    let tenant = tenant_service.find_by_user_id(&session.user_id).await?;

    let feedback = Feedback::new(
        tenant.id,
        &req.property_image,
        &req.customer_image,
        &req.customer_name,
        &req.customer_review,
        &req.description,
    );

    let created_feedback = service.create_feedback(feedback).await?;
    Ok(HttpResponse::Ok().json(created_feedback))
}

pub async fn update_feedback(
    service: web::Data<Arc<Service>>,
    tenant_service: web::Data<Arc<tenant::Service>>,
    lucia_service: web::Data<Arc<lucia::Service>>,
    req: web::Json<CreateUpdateFeedback>,
    req_headers: HttpRequest,
) -> Result<HttpResponse, ApiError> {
    let basic_auth_header = req_headers
        .headers()
        .get("Authorization")
        .and_then(|header_value| header_value.to_str().ok())
        .ok_or_else(|| ApiError::Unauthorized("Missing Authorization header".into()))?;

    let session = lucia_service.get_session(basic_auth_header).await?;
    let tenant = tenant_service.find_by_user_id(&session.user_id).await?;

    let feedback = Feedback::new(
        tenant.id,
        &req.property_image,
        &req.customer_image,
        &req.customer_name,
        &req.customer_review,
        &req.description,
    );

    let updated_feedback = service.update_feedback(feedback).await?;
    Ok(HttpResponse::Ok().json(updated_feedback))
}

pub async fn get_tenant_feedbacks(
    service: web::Data<Arc<Service>>,
    tenant_id: web::Path<i32>,
    query: web::Query<Pagination>,
) -> Result<HttpResponse, ApiError> {
    let feedbacks = service
        .find_tenant_feedback(*tenant_id, query.into_inner())
        .await?;
    Ok(HttpResponse::Ok().json(feedbacks))
}

pub async fn generate_image_presigned_url(
    service: web::Data<Arc<Service>>,
    tenant_service: web::Data<Arc<tenant::Service>>,
    lucia_service: web::Data<Arc<lucia::Service>>,
    req_headers: HttpRequest,
) -> Result<HttpResponse, ApiError> {
    let basic_auth_header = req_headers
        .headers()
        .get("Authorization")
        .and_then(|header_value| header_value.to_str().ok())
        .ok_or_else(|| ApiError::Unauthorized("Missing Authorization header".into()))?;

    let session = lucia_service.get_session(basic_auth_header).await?;
    let tenant = tenant_service.find_by_user_id(&session.user_id).await?;

    let url = service.generate_post_presigned_urls(tenant.id).await?;

    Ok(HttpResponse::Ok().json(PresignedUrlResponse { url }))
}
