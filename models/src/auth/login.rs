use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Validate, Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct LoginModel {
    #[validate(email, length(min = 3, max = 50))]
    pub email_address: String,
    #[validate(length(min = 6, max = 15))]
    pub password: String,
    pub stay_signed_in: bool
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginPageContext {
    pub title: String
}
