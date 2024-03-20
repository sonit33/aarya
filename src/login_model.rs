use rocket::serde::{Deserialize, Serialize};

#[derive(FromForm, Debug, Serialize, Deserialize)]
pub struct LoginModel {
    pub email: String,
    pub password: String,
    pub stay_signed_in: bool,
}

#[derive(FromForm, Debug, Serialize, Deserialize)]
pub struct LoginPageContext {
    pub title: String,
}