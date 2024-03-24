use actix_web::{App, HttpServer, web};
use dotenv::from_filename;
use tera::Tera;

use crate::routes::signup::{signup_get, signup_post};

// use crate::routes::login::{};

mod models;
mod routes;
mod utils;
mod tests;
mod traits;
mod errors;


#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    let env_file = if cfg!(debug_assertions) {
        ".env.dev"
    } else {
        ".env.prod"
    };

    from_filename(env_file).ok();

    let ip = "127.0.0.1";
    let port = 8080;

    println!("Actix running on http://{ip}:{port}");


    let tera = Tera::new("templates/**/*").expect("Failed to initialize Tera");

    HttpServer::new(move || {
        let tera = tera.clone();
        App::new()
            .app_data(web::Data::new(tera))
            .service(signup_post).service(signup_get)
    }).bind((ip, port))?.run().await
}
