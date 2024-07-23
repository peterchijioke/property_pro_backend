use crate::validators::register_validator::RegisterRequest;
use actix_web::{web, HttpResponse, Responder};
use serde::Serialize;
use serde_json::error::Error as SerdeError;
use validator::Validate;

#[derive(Serialize)]
pub struct AuthStruct {
    pub message: String,
    pub status: String,
}

#[derive(Serialize)]
struct IResponder {
    username: String,
    email: String,
}

#[derive(Serialize)]
pub struct ErrorResponse {
    pub error: String,
    pub error_messages: Vec<String>,
}

pub async fn auth_create(req_body: web::Bytes) -> impl Responder {
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

            let responder = IResponder {
                username: "Peter".to_string(),
                email: "peterchijioke1@gmail.com".to_string(),
            };

            HttpResponse::Ok().json(responder)
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
