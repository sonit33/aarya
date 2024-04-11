use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Validate, Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct ForgotPasswordModel {
    #[validate(email, length(min = 3, max = 50))]
    pub email: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForgotPasswordPageContext {
    pub title: String
}
