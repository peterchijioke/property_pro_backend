use regex::Regex;
use serde::Deserialize;
use validator::{Validate, ValidationError};

fn validate_phone_number(phone: &str) -> Result<(), ValidationError> {
    if !phone.chars().all(char::is_numeric) {
        return Err(ValidationError::new("number"));
    }
    Ok(())
}
fn validate_password(password: &str) -> Result<(), ValidationError> {
    let has_uppercase = Regex::new(r"[A-Z]").unwrap();
    let has_lowercase = Regex::new(r"[a-z]").unwrap();
    let has_digit = Regex::new(r"\d").unwrap();
    let has_special = Regex::new(r"[!@#$%^&*(),.?\':{}|<>]").unwrap();

    if !has_uppercase.is_match(password) {
        return Err(ValidationError::new(
            "Password must contain at least one uppercase letter",
        ));
    }
    if !has_lowercase.is_match(password) {
        return Err(ValidationError::new(
            "Password must contain at least one lowercase letter",
        ));
    }
    if !has_digit.is_match(password) {
        return Err(ValidationError::new(
            "Password must contain at least one digit",
        ));
    }
    if !has_special.is_match(password) {
        return Err(ValidationError::new(
            "Password must contain at least one special character",
        ));
    }

    Ok(())
}

#[derive(Deserialize, Validate)]
pub struct RegisterRequest {
    #[validate(length(min = 1))]
    pub first_name: String,

    #[validate(length(min = 1))]
    pub last_name: String,

    #[validate(length(min = 11, message = " Phone number must be 11 digits"))]
    #[validate(custom(function = "validate_phone_number"))]
    pub phone: String,

    #[validate(email)]
    pub email: String,

    #[validate(length(min = 8, message = "Password must be at least 8 characters long"))]
    #[validate(custom(function = "validate_password"))]
    pub password: String,
}
