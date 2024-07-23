use actix_web::web;

use crate::services::auth_service::auth_login;
use crate::services::auth_service::auth_logout;

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/auth")
            .route("/login", web::post().to(auth_login))
            .route("/logout", web::post().to(auth_logout)),
    );
}
