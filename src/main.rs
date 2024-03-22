use actix_web::{App, HttpServer, web};
use tera::Tera;

use crate::route_signup::{signup_get, signup_post};

pub mod model_signup;
pub mod route_signup;
mod model_login;
mod route_login;
mod ops_db;
mod util_random;
mod model_api_response;
mod test_signup;


#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
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
