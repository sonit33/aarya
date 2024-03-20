#[macro_use]
extern crate rocket;

use rocket_dyn_templates::Template;

use crate::login_route::{login_get, login_post};
use crate::signup_route::{signup_get, signup_post};

pub mod signup_model;
pub mod signup_route;
mod login_model;
mod login_route;
mod db_ops;
mod signup_tests;
mod util_random;
mod test_ops;


#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let _rocket = rocket::build()
        .mount("/", routes![signup_get, signup_post, login_post, login_get])
        .mount("/static", rocket::fs::FileServer::from("static"))
        .attach(Template::fairing())
        .launch()
        .await?;

    Ok(())
}

