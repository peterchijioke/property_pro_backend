use crate::db::db::AppState;
use crate::db_operations;
use crate::middleware::auth_middleware::TokenClaims;
use crate::models::user::UserModel;
use crate::utils::hash::{hash_password, verify_password};

use crate::responses::success_response::{SuccessPasswordUpdateResponse, SuccessResponse};
use crate::utils::error_handler::{ApiError, ApiResponse};
use crate::validators::register_validator::{UpdatePassword, UpdateRequest};

use actix_web::{web, HttpMessage, HttpRequest, HttpResponse, Responder};
use hmac::digest::generic_array::functional;
use serde_json::error::Error as SerdeError;
use validator::Validate;

use super::auth_service::ErrorResponse;

pub async fn user_profile(path: web::Path<String>) -> impl Responder {
    match path.parse::<u32>() {
        Ok(_user_id) => {
            let response = ApiResponse {
                status: "success".to_string(),
                message: "Logout successful".to_string(),
            };
            Ok(HttpResponse::Created().json(response))
        }
        Err(e) => Err(ApiError::InvalidRequestData(e.to_string())),
    }
}

pub async fn user_update(
    client: web::Data<AppState>,
    req: HttpRequest,
    req_body: web::Bytes,
) -> Result<HttpResponse, ApiError> {
    let update_req: Result<UpdateRequest, SerdeError> = serde_json::from_slice(&req_body);

    let extensions = req.extensions();
    let token_data = extensions
        .get::<TokenClaims>()
        .ok_or(ApiError::ValidationError(
            "Token data not found".to_string(),
        ))?;
    let user_id = token_data
        .id
        .as_ref()
        .ok_or(ApiError::ValidationError("User ID not found".to_string()))?;
    match update_req {
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
            println!("User ID: {:?}", user_id);
            let user_exists = db_operations::user_db_operations::find_user_by_id(
                &client.db.collection("users"),
                Some(user_id.clone()),
            )
            .await;
            match user_exists {
                None => Err(ApiError::ValidationError(format!(
                    "User with email {} does not exists",
                    register_request.email.as_deref().unwrap_or("unknown")
                ))),

                Some(user) => {
                    let new_user = UserModel {
                        id: user.id.clone(),
                        first_name: (&register_request
                            .first_name
                            .as_deref()
                            .unwrap_or(&user.first_name))
                            .to_string(),
                        last_name: (&register_request
                            .last_name
                            .as_deref()
                            .unwrap_or(&user.last_name))
                            .to_string(),
                        phone: (&register_request.phone.as_deref().unwrap_or(&user.phone))
                            .to_string(),
                        email: (&register_request.email.as_deref().unwrap_or(&user.email))
                            .to_string(),
                        password: user.password,
                    };

                    let updated_user = db_operations::user_db_operations::update_user(
                        &client.db.collection("users"),
                        user.id,
                        new_user,
                    )
                    .await;
                    match updated_user {
                        Some(user_object) => {
                            let response = SuccessResponse {
                                status: "success".to_string(),
                                message: "User updated".to_string(),
                                data: user_object.without_password(),
                            };
                            Ok(HttpResponse::Ok().json(response))
                        }
                        None => Err(ApiError::ValidationError(
                            "User not found or update failed".to_string(),
                        )),
                    }
                }
            }
        }
        Err(e) => Err(ApiError::InvalidRequestData(e.to_string())),
    }
}

pub async fn update_password(
    client: web::Data<AppState>,
    req: HttpRequest,
    req_body: web::Bytes,
) -> Result<HttpResponse, ApiError> {
    let update_req: Result<UpdatePassword, SerdeError> = serde_json::from_slice(&req_body);

    let extensions = req.extensions();
    let token_data = extensions
        .get::<TokenClaims>()
        .ok_or(ApiError::ValidationError(
            "Token data not found".to_string(),
        ))?;
    let user_id = token_data
        .id
        .as_ref()
        .ok_or(ApiError::ValidationError("User ID not found".to_string()))?;
    match update_req {
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
            //=================check and update=======
            let user_exists = db_operations::user_db_operations::find_user_by_id(
                &client.db.collection("users"),
                Some(user_id.clone()),
            )
            .await;
            if let Some(user) = user_exists {
                match verify_password(&register_request.password, &user.password) {
                    Ok(true) => {
                        let hashed_password = hash_password(&register_request.new_password)
                            .map_err(|_| ApiError::PasswordHashingError)?;

                        let new_user = UserModel {
                            password: hashed_password,
                            ..user.clone() // Assuming UserModel implements Clone
                        };

                        let update_result = db_operations::user_db_operations::update_user(
                            &client.db.collection("users"),
                            user.id,
                            new_user,
                        )
                        .await;

                        match update_result {
                            Some(_) => {
                                let response = SuccessPasswordUpdateResponse {
                                    status: "success".to_string(),
                                    message: "Password updated successfully".to_string(),
                                };
                                Ok(HttpResponse::Ok().json(response))
                            }
                            None => Err(ApiError::ValidationError(
                                "User not found or update failed".to_string(),
                            )),
                        }
                    }
                    Ok(false) => Err(ApiError::InvalidRequestData(
                        "Incorrect password".to_string(),
                    )),
                    Err(e) => Err(ApiError::InvalidRequestData(e.to_string())),
                }
            } else {
                Err(ApiError::ValidationError(format!(
                    "User with id {} does not exist",
                    user_id
                )))
            }
        }
        Err(e) => Err(ApiError::InvalidRequestData(e.to_string())),
    }
}
