use actix_web::{App, HttpServer, web};

use crate::route_signup::signup_post;

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
    HttpServer::new(|| {
        App::new()
            .route("/signup", web::post().to(signup_post))
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
