// models/signup_model.rs

use rocket::FromForm;

#[derive(FromForm, Debug)]
pub struct SignupModel {
    pub username: String,
    pub email: String,
    pub password: String,
}
