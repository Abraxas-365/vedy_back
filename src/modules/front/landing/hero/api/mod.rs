use actix_web::web;
use handler::{generate_hero_image_presigned_url, get_tenant_hero, update_hero};

mod handler;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/hero")
            .route("", web::put().to(update_hero))
            .route("/{tenant_id}", web::get().to(get_tenant_hero))
            .route(
                "/image-upload-url",
                web::get().to(generate_hero_image_presigned_url),
            ),
    );
}
