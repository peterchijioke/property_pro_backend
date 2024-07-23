use crate::responses::{error_response::ErrorResponse, success_response::SuccessResponse};
use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct UserData {
    first_name: String,
    last_name: String,
    email: String,
}

pub(crate) async fn user_profile(path: web::Path<String>) -> impl Responder {
    match path.parse::<u32>() {
        Ok(id) => {
            let response = SuccessResponse {
                message: "Successfully retrieved user".to_string(),
                id,
                data: UserData {
                    first_name: "Peter".to_string(),
                    last_name: "Chukwu".to_string(),
                    email: "peterchijioke1@gmail.com".to_string(),
                },
            };
            HttpResponse::Ok().json(response)
        }
        Err(_) => {
            let error_response = ErrorResponse {
                error: "Invalid ID format".to_string(),
                error_messages: vec!["The provided ID could not be parsed as a number.".to_string()],
            };
            HttpResponse::BadRequest().json(error_response)
        }
    }
}

pub(crate) async fn user_update() -> impl Responder {
    let error_response = ErrorResponse {
        error: "Invalid data format".to_string(),
        error_messages: vec!["The provided data format is incorrect.".to_string()],
    };
    HttpResponse::BadRequest().json(error_response)
}
