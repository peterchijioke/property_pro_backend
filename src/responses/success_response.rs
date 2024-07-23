use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct SuccessResponse<T> {
    pub message: String,
    pub id: u32, // Adjust the type based on the actual type of `id`
    pub data: T,
}
