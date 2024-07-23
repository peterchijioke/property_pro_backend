use crate::services::user_service;
use actix_web::web;

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/users")
            .route("", web::get().to(user_service::user_profile))
            .route("/{id}", web::get().to(user_service::user_update)),
    );
}
