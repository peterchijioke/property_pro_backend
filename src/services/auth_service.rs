// src/handlers/user.rs
use actix_web::{web, HttpResponse, Responder};
use mongodb::{bson::doc, bson::oid::ObjectId};
use serde::Serialize;
use serde_json::error::Error as SerdeError;
use validator::Validate;

use crate::db::db::AppState;
use crate::db_operations;
use crate::models::user::UserModel;
use crate::utils::error_handler::{ApiError, ApiResponse, ApiResponseLogin};
use crate::utils::hash::{hash_password, verify_password};
use crate::utils::sign_jwt::create_token;
use crate::validators::register_validator::{LoginRequest, RegisterRequest};

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

pub async fn auth_create(
    client: web::Data<AppState>,
    req_body: web::Bytes,
) -> Result<HttpResponse, ApiError> {
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

                return Ok(HttpResponse::BadRequest().json(ErrorResponse {
                    error: "Validation failed".to_string(),
                    error_messages,
                }));
            }

            let user_exists = db_operations::user_db_operations::find_user_by_email(
                &client.db.collection("users"),
                &register_request.email,
            )
            .await;
            match user_exists {
                Some(_) => Err(ApiError::ValidationError(format!(
                    "User with email {} already exists",
                    register_request.email
                ))),
                None => {
                    let hashed_password = match hash_password(&register_request.password) {
                        Ok(password) => password,
                        Err(_) => return Err(ApiError::PasswordHashingError),
                    };

                    let new_user = UserModel {
                        id: Some(ObjectId::new()),
                        first_name: register_request.first_name.clone(),
                        last_name: register_request.last_name.clone(),
                        phone: register_request.phone.clone(),
                        email: register_request.email.clone(),
                        password: hashed_password,
                    };

                    let user = db_operations::user_db_operations::insert_user(
                        &client.db.collection("users"),
                        new_user,
                    )
                    .await;

                    match &user {
                        Ok(_) => {
                            let response = ApiResponse {
                                status: "success".to_string(),
                                message: "User registered".to_string(),
                            };
                            Ok(HttpResponse::Created().json(response))
                        }
                        Err(e) => Err(ApiError::DatabaseError(e.to_string())),
                    }
                }
            }
        }
        Err(e) => Err(ApiError::InvalidRequestData(e.to_string())),
    }
}

pub async fn auth_login(
    client: web::Data<AppState>,
    req_body: web::Bytes,
) -> Result<HttpResponse, ApiError> {
    let login_req: Result<LoginRequest, SerdeError> = serde_json::from_slice(&req_body);

    match login_req {
        Ok(login_request) => {
            if let Err(errors) = login_request.validate() {
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

                return Ok(HttpResponse::BadRequest().json(ErrorResponse {
                    error: "Validation failed".to_string(),
                    error_messages,
                }));
            }
            let user_exists = db_operations::user_db_operations::find_user_by_email(
                &client.db.collection("users"),
                &login_request.email,
            )
            .await;

            match &user_exists {
                Some(user) => {
                    let hashed_password = &user.password;
                    let user_password = &login_request.password;

                    let is_valid = match verify_password(&user_password, &hashed_password) {
                        Ok(valid) => valid,
                        Err(_) => return Err(ApiError::PasswordHashingError),
                    };

                    if is_valid {
                        let access_token = create_token(&user);
                        match access_token {
                            Ok(token) => {
                                let response = ApiResponseLogin {
                                    status: "success".to_string(),
                                    message: "Login successful".to_string(),
                                    access_token: token,
                                };
                                Ok(HttpResponse::Ok().json(response))
                            }
                            Err(e) => Err(ApiError::InvalidRequestData(e.to_string())),
                        }
                    } else {
                        return Err(ApiError::IncorrectPassword);
                    }
                }
                None => Err(ApiError::ValidationError(format!(
                    "User with email {} does not exists",
                    login_request.email
                ))),
            }
        }
        Err(e) => Err(ApiError::InvalidRequestData(e.to_string())),
    }
}

pub async fn auth_logout(
    client: web::Data<AppState>,
    req_body: web::Bytes,
) -> Result<HttpResponse, ApiError> {
    let logout_req: Result<LoginRequest, SerdeError> = serde_json::from_slice(&req_body);
    let response = ApiResponse {
        status: "success".to_string(),
        message: "Logout successful".to_string(),
    };
    Ok(HttpResponse::Created().json(response))
}
