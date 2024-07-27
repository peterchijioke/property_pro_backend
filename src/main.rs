use actix_web::{web, App, HttpServer};
use db::db::AppState;
use routes::configure_routes;
mod db;
mod db_operations;
mod middleware;
mod models;
mod responses;
mod routes;
mod services;
mod utils;
mod validators;
extern crate dotenv;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let app_state = web::Data::new(AppState::new().await);
    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .configure(configure_routes)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
