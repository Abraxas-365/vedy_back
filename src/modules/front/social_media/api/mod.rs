use actix_web::web;

mod handler;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/social_media")
            .route("/{tenant_id}", web::put().to(upsert_social_media))
            .route("/{tenant_id}", web::get().to(find_social_media)),
    );
}
