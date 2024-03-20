// models/signup_model.rs

use rocket::FromForm;
use serde::{Deserialize, Serialize};

#[derive(FromForm, Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct SignupModel {
    pub id: String,
    pub display_name: String,
    pub email: String,
    pub password: String,
    pub confirm_password: String,
    pub over_13: bool,
}

#[derive(FromForm, Debug, Serialize, Deserialize)]
pub struct SignupPageContext {
    pub title: String,
}
