use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct SuccessResponse<T> {
    pub code: i64,
    pub message: String,
    pub data: Option<T>,
}