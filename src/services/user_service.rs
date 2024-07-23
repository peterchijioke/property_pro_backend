use actix_web::{web, HttpResponse, Responder};
use serde_json::json;

pub(crate) async fn user_profile(path: web::Path<String>) -> impl Responder {
    match path.parse::<u32>() {
        Ok(id) => HttpResponse::Ok().json(json!({
            "message": "From user profile",
            "id": id
        })),
        Err(_) => HttpResponse::BadRequest().json(json!({
            "error": "Invalid ID format"
        })),
    }
}

pub(crate) async fn user_update() -> impl Responder {
    HttpResponse::Ok().json(json!({
        "message": "From user update",
    }))
}
