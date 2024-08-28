#[macro_use]
pub mod macros;
pub mod api;
pub mod entities;
pub mod models;
pub mod pages;
pub mod utils;

use actix_cors::Cors;
use actix_files as fs;
use actix_web::{http, middleware, web, App, HttpServer};
use api::{
    files::post_markdown,
    photos::{post_photo, post_photos},
};
use dotenv::from_filename;
use handlebars::Handlebars;
use mongodb::Client;
use pages::{
    admin::{
        author::{
            get_author_list, get_create_author, get_edit_author, post_create_author,
            post_edit_author,
        },
        post::{get_create_post, get_edit_post, get_post_list, post_create_post, post_edit_post},
        tag::{get_create_tag, get_edit_tag, get_tag_list, post_create_tag, post_edit_tag},
    },
    blogs::{get_post, get_posts},
};
use utils::{environ::Environ, file_ops::read_files_from_dir};

fn configure_handlebars() -> Handlebars<'static> {
    let mut handlebars = Handlebars::new();
    read_files_from_dir("./pages", ".hbs")
        .iter()
        .for_each(|file| {
            handlebars
                .register_template_file(&file.name, &file.path)
                .unwrap()
        });

    handlebars
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let env_file = if cfg!(debug_assertions) {
        ".env.dev"
    } else {
        ".env.prod"
    };

    from_filename(env_file).ok();

    env_logger::init_from_env(env_logger::Env::new().default_filter_or("debug"));

    let env_default = Environ::default();

    log::debug!("{:?}", env_default);

    let mongoc = Client::with_uri_str(env_default.db_connection_string)
        .await
        .unwrap();

    let ip = "localhost";
    let port = env_default.web_app_port;

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
            .app_data(web::Data::new(mongoc.clone()))
            .service(fs::Files::new("/assets", "./assets").show_files_listing())
            .service(get_posts)
            .service(get_create_post)
            .service(post_create_post)
            .service(get_create_author)
            .service(post_create_author)
            .service(get_create_tag)
            .service(post_create_tag)
            .service(get_tag_list)
            .service(get_edit_tag)
            .service(post_edit_tag)
            .service(post_photo)
            .service(post_photos)
            .service(post_markdown)
            .service(get_author_list)
            .service(get_edit_author)
            .service(post_edit_author)
            .service(get_post_list)
            .service(get_edit_post)
            .service(post_edit_post)
            .service(get_post)
    })
    .bind((ip, port))?
    .run()
    .await
}
