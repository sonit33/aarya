use rocket::response::Redirect;
use rocket_dyn_templates::Template;

use crate::signup_model::{SignupModel, SignupPageContext};

#[get("/signup")]
pub fn signup_get() -> Template {
    let context = SignupPageContext {
        title: String::from("Signup")
    };
    Template::render(
        "auth_signup",
        &context,
    )
}

#[post("/signup", data = "<form_data>")]
pub fn signup_post(form_data: rocket::form::Form<SignupModel>) -> Redirect {
    // Here, you would handle the signup logic, such as saving the user to a database.
    // For demonstration, we'll redirect to the home page.
    println!("{:?}", form_data);

    Redirect::to(uri!("/login"))
}



