use serde::Deserialize;
use validator::{Validate, ValidationError};

fn validate_phone_number(phone: &str) -> Result<(), ValidationError> {
    if !phone.chars().all(char::is_numeric) {
        return Err(ValidationError::new("number"));
    }
    Ok(())
}

#[derive(Deserialize, Validate)]
pub struct RegisterRequest {
    #[validate(length(min = 1))]
    pub first_name: String,

    #[validate(length(min = 1))]
    pub last_name: String,

    #[validate(length(min = 1), custom(function = "validate_phone_number"))]
    pub phone: String,

    #[validate(email)]
    pub email: String,
}
