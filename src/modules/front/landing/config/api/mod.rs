use actix_web::web;
use handler::{generate_logo_presigned_url, get_tenant_config, update_config};

mod handler;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/config")
            .route("", web::put().to(update_config))
            .route("/tenant/{tenant_id}", web::get().to(get_tenant_config))
            .route(
                "/logo-upload-url",
                web::get().to(generate_logo_presigned_url),
            ),
    );
}
