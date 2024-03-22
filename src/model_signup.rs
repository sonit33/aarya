// models/model_signup

use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationError};

#[derive(Validate, Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct SignupModel {
    #[validate(length(min = 3, max = 50))]
    pub id: String,
    #[validate(length(min = 3, max = 50))]
    pub display_name: String,
    #[validate(email, custom(function = "is_unique"))]
    pub email: String,
    #[validate(length(min = 3, max = 15))]
    pub password: String,
    #[validate(length(min = 3, max = 15))]
    #[validate(must_match(other = "password"))]
    pub confirm_password: String,
    #[validate(custom(function = "must_be_true"))]
    pub over_13: bool,
}

fn is_unique(email: &str) -> Result<(), ValidationError> {
    if email == "aaa@aa.com" {
        // the value of the username will automatically be added later
        return Err(ValidationError::new("bad_email"));
    }

    Ok(())
}

fn must_be_true(over_13: &bool) -> Result<(), ValidationError> {
    if !over_13 {
        return Err(ValidationError::new("You must be over 13 to use this app"));
    }

    Ok(())
}
