use actix_web::{App, HttpServer};
use routes::configure_routes;
mod middleware;
mod models;
mod responses;
mod routes;
mod services;
mod validators;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().configure(configure_routes))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
