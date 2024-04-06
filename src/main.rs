use actix_web::{ App, HttpServer, web };
use dotenv::from_filename;
use sqlx::MySqlPool;
use tera::Tera;

use crate::routes::auth::forgot_password::{
    forgot_password_email_post,
    forgot_password_email_sent_get,
    forgot_password_get,
};
use crate::routes::auth::login::{ login_get, login_post };
use crate::routes::auth::reset_password::{ reset_password_get, reset_password_post };
use crate::routes::auth::signup::{ signup_get, signup_post };
use crate::routes::auth::activate_account::{
    account_activate_get,
    activate_account_email_sent_get,
    activate_account_get,
    activate_account_post,
};
use crate::utils::email_sender::EmailSender;
use crate::utils::environ::Environ;

mod models;
mod routes;
pub mod utils;
#[cfg(test)]
mod tests;
mod traits;
mod errors;
mod macros;

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    let env_file = if cfg!(debug_assertions) { ".env.dev" } else { ".env.prod" };

    from_filename(env_file).ok();

    env_logger::init_from_env(env_logger::Env::new().default_filter_or("debug"));

    let ip = "127.0.0.1";
    let port = 8080;

    let env_default = Environ::default();

    log::debug!("{:?}", env_default);

    let database_url = env_default.db_connection_string;
    let pool = MySqlPool::connect(database_url.as_str()).await.expect(
        "Failed to connect to database"
    );
    let tera = Tera::new("templates/**/*").expect("Failed to initialize Tera");
    // let e_port: Result<u16, _> = env_default.email_port.parse();
    let email_sender = EmailSender {};

    println!("Actix running at http://{ip}:{port}");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(tera.clone()))
            .app_data(web::Data::new(pool.clone()))
            .app_data(web::Data::new(email_sender.clone()))
            .service(signup_post)
            .service(signup_get)
            .service(login_get)
            .service(login_post)
            .service(forgot_password_get)
            .service(forgot_password_email_post)
            .service(forgot_password_email_sent_get)
            .service(activate_account_get)
            .service(activate_account_post)
            .service(account_activate_get)
            .service(activate_account_email_sent_get)
            .service(reset_password_get)
            .service(reset_password_post)
    })
        .bind((ip, port))?
        .run().await
}
