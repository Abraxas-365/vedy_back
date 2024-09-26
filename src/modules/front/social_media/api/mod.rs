use actix_web::web;
use handler::{find_social_media, upsert_social_media};

mod handler;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/social_media")
            .route("/{tenant_id}", web::put().to(upsert_social_media))
            .route("/{tenant_id}", web::get().to(find_social_media)),
    );
}
