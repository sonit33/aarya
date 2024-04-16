use aarya_utils::environ::Environ;
use actix_web::{ web, App, HttpServer };
use dotenv::from_filename;
use sqlx::MySqlPool;

use crate::services::question_service::*;

pub mod entities;
pub mod services;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let env_file = if cfg!(debug_assertions) { ".env.dev" } else { ".env.prod" };

    from_filename(env_file).ok();

    let ip = "127.0.0.1";
    let port = 8080;

    env_logger::init_from_env(env_logger::Env::new().default_filter_or("debug"));

    let env_default = Environ::default();

    log::debug!("{:?}", env_default);

    let database_url = format!("{}/{}", env_default.db_connection_string, env_default.db_name);
    let pool = MySqlPool::connect(database_url.as_str()).await.expect(
        "Failed to connect to database"
    );

    println!("Actix running at http://{ip}:{port}");

    HttpServer::new(move || {
        App::new()
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
        .run().await
}
