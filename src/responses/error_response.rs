use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ErrorResponse {
    pub error: String,
    pub(crate) error_messages: Vec<String>,
}
