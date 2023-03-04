use actix_web::{App, HttpServer};
use actix_web::middleware::Logger;
use actix_web::web::{Data, JsonConfig};
use crate::config::conf::{CONFIG, start_tracing};

use crate::controller::routes;

mod controller;
mod config;
mod model;
mod error;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    start_tracing();
    dotenv::dotenv().expect("Failed to read .env file");

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(Data::new(JsonConfig::default().limit(4096)))
            .configure(routes)
    })
        .bind(format!("{}:{}", CONFIG.host, CONFIG.rest_port))
        .expect("REST Server binding exception")
        .run()
        .await
}