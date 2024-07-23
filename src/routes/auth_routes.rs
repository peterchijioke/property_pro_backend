use actix_web::web;

use crate::services::auth_service::{auth_create, auth_login, auth_logout};

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/auth")
            .route("/login", web::post().to(auth_login))
            .route("/register", web::post().to(auth_create))
            .route("/logout", web::post().to(auth_logout)),
    );
}
