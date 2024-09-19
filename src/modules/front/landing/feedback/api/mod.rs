use actix_web::web;
use handler::{
    create_feedback, delete_feedback, generate_image_presigned_url, get_tenant_feedbacks,
    update_feedback,
};

mod handler;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/feedback")
            .route("", web::post().to(create_feedback))
            .route("", web::put().to(update_feedback))
            .route("/{tenant_id}", web::get().to(get_tenant_feedbacks))
            .route(
                "/image-upload-url",
                web::get().to(generate_image_presigned_url),
            )
            .route("/{feedback_id}", web::delete().to(delete_feedback)),
    );
}
