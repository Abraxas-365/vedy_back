use actix_web::{web, HttpRequest, HttpResponse};
use serde::Deserialize;
use std::sync::Arc;

use crate::error::ApiError;
use crate::modules::front::social_media::{Service, SocialMedia};
use crate::modules::tenant::Service as TenantService;
use crate::utils::lucia::Service as LuciaService;

#[derive(Deserialize)]
pub struct UpsertSocialMediaRequest {
    pub facebook_url: Option<String>,
    pub instagram_url: Option<String>,
    pub tiktok_url: Option<String>,
    pub linkedin_url: Option<String>,
}

pub async fn upsert_social_media(
    service: web::Data<Arc<Service>>,
    tenant_service: web::Data<Arc<TenantService>>,
    lucia_service: web::Data<Arc<LuciaService>>,
    req: web::Json<UpsertSocialMediaRequest>,
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

    let social_media = SocialMedia {
        id: 0, // This will be ignored by the database
        tenant_id: tenant.id,
        facebook_url: req.facebook_url.clone(),
        instagram_url: req.instagram_url.clone(),
        tiktok_url: req.tiktok_url.clone(),
        linkedin_url: req.linkedin_url.clone(),
    };

    let upserted_social_media = service.upsert(social_media, tenant.id).await?;
    Ok(HttpResponse::Ok().json(upserted_social_media))
}

pub async fn find_social_media(
    service: web::Data<Arc<Service>>,
    tenant_service: web::Data<Arc<TenantService>>,
    lucia_service: web::Data<Arc<LuciaService>>,
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

    let social_media = service.find(tenant.id).await?;
    Ok(HttpResponse::Ok().json(social_media))
}
