use rocket::response::Redirect;
use rocket_dyn_templates::Template;

use crate::login_model::{LoginModel, LoginPageContext};

#[get("/login")]
pub fn login_get() -> Template {
    let context = LoginPageContext {
        title: String::from("Login")
    };
    Template::render(
        "auth_login",
        &context,
    )
}

#[post("/login", data = "<form_data>")]
pub fn login_post(form_data: rocket::form::Form<LoginModel>) -> Redirect {
    // Here, you would handle the signup logic, such as saving the user to a database.
    // For demonstration, we'll redirect to the home page.
    println!("{:?}", form_data);

    Redirect::to(uri!("/"))
}

