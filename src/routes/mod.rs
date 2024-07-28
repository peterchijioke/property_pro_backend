pub mod auth_routes;
pub mod user_routes;

use actix_web::web;
pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    user_routes::configure_routes(cfg);
    auth_routes::configure_routes(cfg);
}
