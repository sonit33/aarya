use actix_web::{ App, HttpServer, web };
use dotenv::from_filename;
use sqlx::MySqlPool;
use tera::Tera;

use crate::routes::auth::forgot_password::{ forgot_password_email_get, forgot_password_email_post };
use crate::routes::auth::login::{ login_get, login_post };
use crate::routes::auth::reset_password::reset_password_get;
use crate::routes::auth::signup::{ signup_get, signup_post };
use crate::routes::auth::verify_email::{ verify_email_get, verify_email_post };
use crate::utils::email_sender::EmailSender;
use crate::utils::environ::Environ;

mod models;
mod routes;
pub mod utils;
#[cfg(test)]
mod tests;
mod traits;
mod errors;

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    let env_file = if cfg!(debug_assertions) { ".env.dev" } else { ".env.prod" };

    from_filename(env_file).ok();

    let ip = "127.0.0.1";
    let port = 8080;

    println!("Actix running on http://{ip}:{port}");

    let env_default = Environ::default();

    let database_url = env_default.db_connection_string;
    let pool = MySqlPool::connect(database_url.as_str()).await.expect(
        "Failed to connect to database"
    );
    let tera = Tera::new("templates/**/*").expect("Failed to initialize Tera");
    let e_port: Result<u16, _> = env_default.email_port.parse();
    let email_sender = EmailSender::new(
        env_default.email_server,
        e_port.unwrap(),
        env_default.email_username,
        env_default.email_password
    );

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(tera.clone()))
            .app_data(web::Data::new(pool.clone()))
            .app_data(web::Data::new(email_sender.clone()))
            .service(signup_post)
            .service(signup_get)
            .service(login_get)
            .service(login_post)
            .service(forgot_password_email_get)
            .service(forgot_password_email_post)
            .service(verify_email_get)
            .service(verify_email_post)
            .service(reset_password_get)
    })
        .bind((ip, port))?
        .run().await
}
