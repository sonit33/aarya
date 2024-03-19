#[macro_use] extern crate rocket;

use rocket_dyn_templates::Template;

mod models {
    pub mod signup_model;
}

mod routes {
    pub mod signup_route;
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![routes::signup_route::index, routes::signup_route::signup])
        .attach(Template::fairing())
}
