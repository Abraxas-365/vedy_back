use actix_web::web;
use handler::{
    create_property, delete_property, generate_presigned_urls, get_property_by_id,
    get_tenant_properties, update_property,
};

mod handler;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/properties")
            .route("", web::post().to(create_property))
            .route("/{property_id}", web::get().to(get_property_by_id))
            .route("/{property_id}", web::delete().to(delete_property))
            .route("{property_id}", web::put().to(update_property)),
    )
    .service(
        web::scope("/tenants/{tenant_id}")
            .route("/properties", web::get().to(get_tenant_properties))
            .route(
                "/generate_presigned_urls",
                web::post().to(generate_presigned_urls),
            ),
    );
}
