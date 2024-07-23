use crate::middleware::auth_middleware;
use crate::services::user_service;

use actix_web::web;

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/users")
            .wrap(auth_middleware::AuthMiddleware)
            .route("", web::get().to(user_service::user_update))
            .route("/{id}", web::get().to(user_service::user_profile)),
    );
}
