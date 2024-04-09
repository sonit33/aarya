use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Validate, Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct VerifyEmailModel {
    #[validate(email, length(min = 3, max = 50))]
    pub email_address: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VerifyEmailPageContext {
    pub title: String
}
