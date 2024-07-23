use actix_web::{web, HttpResponse, Responder};

pub(crate) async fn user_profile(path: web::Path<u32>) -> impl Responder {
    let id = path.into_inner();
    HttpResponse::Ok().body(format!("User profile for user {}", id))
}

pub(crate) async fn user_update() -> impl Responder {
    HttpResponse::Ok().body("User index")
}
