use std::sync::Arc;

use actix_web::{web, App, HttpServer};
use routes::configure_routes;
mod db;
mod middleware;
mod models;
mod responses;
mod routes;
mod services;
mod utils;
mod validators;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let mongo_client = db::get_mongo_client()
        .await
        .expect("Failed to initialize MongoDB client");
    let mongo_client = Arc::new(mongo_client);

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(mongo_client.clone()))
            .configure(configure_routes)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
