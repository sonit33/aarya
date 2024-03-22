use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ApiResponseModel<T: Serialize> {
    pub message: String,
    pub payload: T,
}