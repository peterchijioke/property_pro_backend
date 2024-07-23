use actix_web::{App, HttpServer};
use routes::configure_routes;
mod models;
mod routes;
mod services;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().configure(configure_routes) // Register the routes
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
