use dotenv::dotenv;
use hmac::{Hmac, Mac};
use jwt::{AlgorithmType, Header, SignWithKey, Token};
use sha2::Sha256;
use std::collections::BTreeMap;
use std::env;

use crate::models::user::UserModel;

pub fn create_token(user: &UserModel) -> Result<String, Box<dyn std::error::Error>> {
    let _ = user;
    dotenv().ok();
    let secret_key = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    let key: Hmac<Sha256> = Hmac::new_from_slice(secret_key.as_bytes()).unwrap();

    let header = Header {
        algorithm: AlgorithmType::Hs256,
        ..Default::default()
    };
    let mut claims = BTreeMap::new();
    claims.insert("id".to_string(), user.id);

    let token_result = Token::new(header, claims).sign_with_key(&key);
    let token = match token_result {
        Ok(t) => t,
        Err(e) => return Err(Box::new(e)),
    };

    Ok(token.into())
}
