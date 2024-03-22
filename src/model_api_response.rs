use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ApiResponseModel<T: Serialize> {
    pub success: bool,
    pub message: String,
    pub status_code: usize,
    pub payload: T,
}