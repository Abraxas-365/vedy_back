use actix_web::web;
use handler::update_tenant;

mod handler;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/tenant").route("/{id}", web::put().to(update_tenant)));
}
