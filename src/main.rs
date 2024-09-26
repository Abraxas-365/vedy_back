#![allow(dead_code)]

use std::sync::Arc;

use actix_cors::Cors;
use actix_web::{middleware::Logger, web, App, HttpServer};
use modules::{
    front::{
        landing::{
            config::{self},
            feedback, hero,
        },
        social_media,
    },
    property, stats, tenant,
};
use utils::s3;

use crate::utils::database::PostgresRepository;

mod error;
mod modules;
mod utils;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "info,debug");
    env_logger::init();

    let repo = Arc::new(PostgresRepository::new().await);
    let bukcet_service = Arc::new(s3::S3Repository::new().await.unwrap());
    let property_service = Arc::new(property::Service::new(repo.clone(), bukcet_service.clone()));
    let tenant_service = Arc::new(tenant::Service::new(repo.clone()));
    let hero_service = Arc::new(hero::Service::new(repo.clone(), bukcet_service.clone()));
    let config_service = Arc::new(config::Service::new(repo.clone(), bukcet_service.clone()));
    let feedback_service = Arc::new(feedback::Service::new(repo.clone(), bukcet_service.clone()));
    let stats_service = Arc::new(stats::Service::new(repo.clone()));
    let social_service = Arc::new(social_media::Service::new(repo.clone()));
    let luci_service = Arc::new(utils::lucia::Service::new(repo.clone()));

    log::info!("Starting HTTP server on 0.0.0.0:80...");
    HttpServer::new(move || {
        let cors = Cors::permissive();

        App::new()
            .wrap(cors)
            .wrap(Logger::default())
            .service(
                web::scope("/v2")
                    .configure(property::config)
                    .configure(hero::config)
                    .configure(config::config)
                    .configure(feedback::config)
                    .configure(tenant::config)
                    .configure(stats::config)
                    .configure(social_media::config),
            )
            .app_data(web::Data::new(stats_service.clone()))
            .app_data(web::Data::new(luci_service.clone()))
            .app_data(web::Data::new(property_service.clone()))
            .app_data(web::Data::new(social_service.clone()))
            .app_data(web::Data::new(tenant_service.clone()))
            .app_data(web::Data::new(config_service.clone()))
            .app_data(web::Data::new(feedback_service.clone()))
            .app_data(web::Data::new(hero_service.clone()))
    })
    .bind("0.0.0.0:3000")?
    .run()
    .await
}
