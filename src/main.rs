#[macro_use]
extern crate rocket;

use rocket_dyn_templates::Template;

use crate::signup_route::{signup_get, signup_post};

pub mod signup_model;
pub mod signup_route;

mod models {}

mod routes {}

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let _rocket = rocket::build()
        .mount("/", routes![signup_get, signup_post])
        .mount("/static", rocket::fs::FileServer::from("static"))
        .attach(Template::fairing())
        .launch()
        .await?;

    Ok(())
}

