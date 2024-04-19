use aarya_utils::{environ::Environ, file_ops::read_files_from_dir};
use actix_cors::Cors;
use actix_web::{http, middleware, web, App, HttpServer};
use dotenv::from_filename;
use handlebars::Handlebars;
use sqlx::MySqlPool;

use crate::{
    routes::index::home,
    services::{
        courses::get_all_courses,
        questions::{
            delete_question_by_id, get_all_questions, get_question_by_deduplicating_hash, get_questions_by_chapter, get_questions_by_chapter_course, get_questions_by_course, get_questions_by_id_hash,
            question_create, update_question_by_id,
        },
    },
};

pub mod entities;
pub mod routes;
pub mod services;

fn configure_handlebars() -> Handlebars<'static> {
    let mut handlebars = Handlebars::new();
    read_files_from_dir("./templates", ".hbs")
        .iter()
        .for_each(|file| handlebars.register_template_file(&file.name, &file.path).unwrap());

    handlebars
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let env_file = if cfg!(debug_assertions) { ".env.dev" } else { ".env.prod" };

    from_filename(env_file).ok();

    let ip = "localhost";
    let port = 9090;

    env_logger::init_from_env(env_logger::Env::new().default_filter_or("debug"));

    let env_default = Environ::default();

    log::debug!("{:?}", env_default);

    let database_url = format!("{}/{}", env_default.db_connection_string, env_default.db_name);
    let pool = MySqlPool::connect(database_url.as_str()).await.expect("Failed to connect to database");

    println!("Actix running at http://{ip}:{port}");

    let handlebars = configure_handlebars();

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin(env_default.allowed_origin.as_str())
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
            .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
            .max_age(3600);

        App::new()
            .wrap(middleware::Logger::default())
            .wrap(cors)
            .app_data(web::Data::new(handlebars.clone()))
            .app_data(web::Data::new(pool.clone()))
            .service(question_create)
            .service(get_all_questions)
            .service(get_all_courses)
            .service(get_questions_by_id_hash)
            .service(get_questions_by_chapter)
            .service(get_questions_by_course)
            .service(get_questions_by_chapter_course)
            .service(get_question_by_deduplicating_hash)
            .service(update_question_by_id)
            .service(delete_question_by_id)
            .service(home)
    })
    .bind((ip, port))?
    .run()
    .await
}
