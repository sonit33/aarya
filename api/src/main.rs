use aarya_utils::environ::Environ;
use actix_cors::Cors;
use actix_web::{http, middleware, web, App, HttpServer};
use dotenv::from_filename;
use sqlx::MySqlPool;

use crate::services::question_service::*;

pub mod entities;
pub mod services;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let env_file = if cfg!(debug_assertions) { ".env.dev" } else { ".env.prod" };

    from_filename(env_file).ok();

    let ip = "localhost";
    let port = 8080;

    env_logger::init_from_env(env_logger::Env::new().default_filter_or("debug"));

    let env_default = Environ::default();

    log::debug!("{:?}", env_default);

    let database_url = format!("{}/{}", env_default.db_connection_string, env_default.db_name);
    let pool = MySqlPool::connect(database_url.as_str()).await.expect("Failed to connect to database");

    println!("Actix running at http://{ip}:{port}");

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin(env_default.allowed_origin.as_str())
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
            .max_age(3600);

        App::new()
            .wrap(middleware::Logger::default())
            .wrap(cors)
            .app_data(web::Data::new(pool.clone()))
            .service(question_create)
            .service(get_all_questions)
            .service(get_questions_by_id_hash)
            .service(get_questions_by_chapter)
            .service(get_questions_by_course)
            .service(get_questions_by_chapter_course)
            .service(get_question_by_deduplicating_hash)
            .service(update_question_by_id)
            .service(delete_question_by_id)
    })
    .bind((ip, port))?
    .run()
    .await
}
