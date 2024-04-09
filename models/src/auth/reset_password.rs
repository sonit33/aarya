use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Validate, Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct ResetPasswordModel {
    #[validate(length(min = 6, max = 20))]
    pub password: String,
    #[validate(length(min = 6, max = 15))]
    #[validate(must_match(other = "password"))]
    pub confirm_password: String,
    // #[validate(length(equal = 8))]
    pub email_hash: String
}
