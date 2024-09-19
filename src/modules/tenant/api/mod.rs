use actix_web::web;
use handler::{get_tenant_by_id, update_tenant};

mod handler;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/tenant")
            .route("/{id}", web::put().to(update_tenant))
            .route("/{id}", web::get().to(get_tenant_by_id)),
    );
}
