use actix_web::{dev::ServiceRequest, Error, HttpMessage, HttpResponse, Result};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use hmac::{Hmac, Mac};
use jwt::VerifyWithKey;
use serde::{Deserialize, Serialize};
use sha2::Sha256;
use std::env;

#[derive(Serialize, Deserialize, Clone)]
pub struct TokenClaims {
    id: i32,
}

pub async fn auth_middleware(
    req: ServiceRequest,
    credentials: BearerAuth,
) -> Result<ServiceRequest, (Error, ServiceRequest)> {
    let secret_key = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    let key: Hmac<Sha256> = Hmac::new_from_slice(secret_key.as_bytes()).unwrap();
    let token_string = credentials.token();

    let claims: Result<TokenClaims, &str> = token_string
        .verify_with_key(&key)
        .map_err(|_| "Invalid token");

    match claims {
        Ok(value) => {
            req.extensions_mut().insert(value);
            Ok(req)
        }
        Err(_) => Err((actix_web::error::ErrorUnauthorized("Invalid token"), req)),
    }
}
