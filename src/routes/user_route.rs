use crate::middleware::auth_middleware::auth_middleware;
use crate::services::user_service;

use actix_web::web;
use actix_web_httpauth::middleware::HttpAuthentication;

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    let bearer_middleware = HttpAuthentication::bearer(auth_middleware);
    cfg.service(
        web::scope("/users")
            .wrap(bearer_middleware)
            .route("", web::get().to(user_service::user_update))
            .route("/{id}", web::get().to(user_service::user_profile)),
    );
}
