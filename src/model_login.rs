use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginModel {
    pub email: String,
    pub password: String,
    pub stay_signed_in: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginPageContext {
    pub title: String,
}