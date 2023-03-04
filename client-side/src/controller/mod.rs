use actix_web::{HttpResponse, web};
use actix_web::web::ServiceConfig;

use crate::controller::welcome_controller::welcome_student_client_api;

mod welcome_controller;

pub fn routes(config: &mut ServiceConfig) {
    config
        .route("/health", web::get().to(|| async { HttpResponse::Ok().json("Hello World!!") }))
        .service(
            web::scope("/client")
                .route("/welcome", web::get().to(welcome_student_client_api))
        );
}