use actix_web::{web, Responder};
use serde::Serialize;

#[derive(Serialize)]
pub struct AuthStruct {
    pub message: String,
    pub status: String,
}

pub async fn auth_login() -> impl Responder {
    let response = AuthStruct {
        message: "Successful".to_string(),
        status: "true".to_string(),
    };

    web::Json(response)
}

pub async fn auth_logout() -> impl Responder {
    let response = AuthStruct {
        message: "Logged out".to_string(),
        status: "true".to_string(),
    };

    web::Json(response)
}
