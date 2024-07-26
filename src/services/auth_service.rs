use std::sync::Arc;

use crate::models::user::{UserModel, UserNoPassword};
use crate::utils::hash;
use crate::validators::register_validator::RegisterRequest;
use actix_web::{web, HttpResponse, Responder};
use mongodb::Client;
use serde::Serialize;
use serde_json::error::Error as SerdeError;
use validator::Validate;

#[derive(Serialize)]
pub struct AuthStruct {
    pub message: String,
    pub status: String,
}

#[derive(Serialize)]
pub struct ErrorResponse {
    pub error: String,
    pub error_messages: Vec<String>,
}

pub async fn auth_create(client: web::Data<Arc<Client>>, req_body: web::Bytes) -> impl Responder {
    let register_req: Result<RegisterRequest, SerdeError> = serde_json::from_slice(&req_body);
    match register_req {
        Ok(register_request) => {
            if let Err(errors) = register_request.validate() {
                let error_messages: Vec<String> = errors
                    .field_errors()
                    .iter()
                    .flat_map(|(field, errs)| {
                        errs.iter().map(move |err| {
                            let error_message = err.message.clone().unwrap_or_default().to_string();
                            format!("{}: {}", field, error_message)
                        })
                    })
                    .collect();
                let error_response = ErrorResponse {
                    error: "Validation failed".to_string(),
                    error_messages,
                };
                return HttpResponse::BadRequest().json(error_response);
            }
            let new_user = UserModel {
                first_name: register_request.first_name.clone(),
                last_name: register_request.last_name.clone(),
                phone: register_request.phone.clone(),
                email: register_request.email.clone(),
                password: "".to_string(),
            };

            let result = UserModel::create(&client, new_user).await;

            match result {
                Ok(_) => {
                    let responder = UserNoPassword {
                        first_name: register_request.first_name.clone(),
                        last_name: register_request.last_name.clone(),
                        email: register_request.email.clone(),
                        phone: register_request.phone.clone(),
                    };
                    HttpResponse::Ok().json(responder)
                }
                Err(e) => HttpResponse::InternalServerError()
                    .body(format!("Failed to create user: {:?}", e)),
            }
        }
        Err(e) => {
            let error_response = ErrorResponse {
                error: "Invalid request data".to_string(),
                error_messages: vec![e.to_string()],
            };
            HttpResponse::BadRequest().json(error_response)
        }
    }
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
