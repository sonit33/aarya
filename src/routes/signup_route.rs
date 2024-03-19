// routes/signup_route.rs

use rocket::form::Form;
use rocket::{get, post};
use rocket_dyn_templates::{Template, context};
use crate::models::signup_model::SignupModel;

#[get("/signup")]
pub fn index() -> Template {
    Template::render("signup", context!{field:"value"})
}

#[post("/signup", data = "<signup_data>")]
pub fn signup(signup_data: Form<SignupModel>) -> String {
    format!("Welcome, {}! You have signed up with the email: {}", signup_data.username, signup_data.email)
}
