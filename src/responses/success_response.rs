use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct SuccessResponse<T> {
    pub status: String,
    pub message: String,
    pub data: T,
}

#[derive(Serialize, Deserialize)]
pub struct SuccessPasswordUpdateResponse {
    pub status: String,
    pub message: String,
}
