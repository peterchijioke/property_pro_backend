use actix_web::web;

use crate::services::auth_service::{auth_create, auth_login, auth_logout};

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/v1/auth")
            .route("/register", web::post().to(auth_create))
            .route("/login", web::post().to(auth_login))
            .route("/logout", web::post().to(auth_logout)),
    );
}
