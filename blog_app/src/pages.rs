use aarya_utils::hash_ops::string_hasher;
use actix_web::{get, web, HttpResponse, Responder};
use handlebars::Handlebars;
use models::{
    blogs::{PostEntity, PostQueryModel},
    result_types::EntityResult,
};
use pulldown_cmark::{html, Parser};
use serde_json::json;
use sqlx::MySqlPool;

#[get("/")]
pub async fn home_page(
    handlebars: web::Data<Handlebars<'_>>,
    pool: web::Data<MySqlPool>,
) -> impl Responder {
    // load blog posts
    let post = PostEntity::default();
    match post.find(&pool).await {
        EntityResult::Success(p) => {
            render_template!(handlebars, "index", json!({"title": "Aarya blog", "posts": p}))
        }
        EntityResult::Error(e) => HttpResponse::InternalServerError().body(format!("Error retrieving posts: [{:?}]", e)),
    }
}

#[get("/{date_published}/{title}")]
pub async fn post_page(
    handlebars: web::Data<Handlebars<'_>>,
    pool: web::Data<MySqlPool>,
    path: web::Path<(String, String)>,
) -> impl Responder {
    let (date_published, title) = path.into_inner();
    let post_hash = string_hasher(format!("{}/{}", date_published, title).as_str());
    let post = PostEntity {
        post_hash: Some(post_hash),
        ..Default::default()
    };

    match post.find_post(&pool).await {
        EntityResult::Success(p) => {
            let mut html = String::new();
            html::push_html(&mut html, Parser::new(&p.post_body));
            let post = PostQueryModel { post_body: html, ..p };
            render_template!(handlebars, "post", json!({"title": "Aarya blog", "post": post}))
        }
        EntityResult::Error(e) => HttpResponse::InternalServerError().body(format!("Error retrieving posts: [{:?}]", e)),
    }
}
