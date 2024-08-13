use crate::middleware::auth_middleware::auth_middleware;
use crate::services::user_service::{update_password, user_profile, user_update};

use actix_web::web;
use actix_web_httpauth::middleware::HttpAuthentication;

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    let bearer_middleware = HttpAuthentication::bearer(auth_middleware);
    cfg.service(
        web::scope("/api/v1/users")
            .wrap(bearer_middleware)
            .route("/update", web::put().to(user_update))
            .route("/update/password", web::put().to(update_password))
            .route("/{id}", web::get().to(user_profile)),
    );
}
