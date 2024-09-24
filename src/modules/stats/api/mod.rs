mod handler;

use actix_web::web;
use handler::{create_stats, get_landing_visited_info, get_property_visited_info};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/stats")
            .route("", web::post().to(create_stats))
            .service(
                web::scope("/tenants/{tenant_id}")
                    .route(
                        "/property_visited",
                        web::get().to(get_property_visited_info),
                    )
                    .route("/landing_visited", web::get().to(get_landing_visited_info)),
            ),
    );
}
