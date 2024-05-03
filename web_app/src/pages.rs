use actix_web::{get, web, HttpResponse, Responder};
use handlebars::Handlebars;
use models::{courses::CourseEntity, result_types::EntityResult};
use serde_json::json;
use sqlx::MySqlPool;

#[get("/")]
pub async fn home_page(handlebars: web::Data<Handlebars<'_>>) -> impl Responder {
    render_template!(handlebars, "index", json!({"title": "Aarya welcomes you!"}))
}

#[get("/configure-test")]
pub async fn test_config_page(
    handlebars: web::Data<Handlebars<'_>>,
    pool: web::Data<MySqlPool>,
) -> impl Responder {
    let course = CourseEntity::new();
    let courses = match course.find_courses(&pool).await {
        EntityResult::Success(courses) => courses,
        EntityResult::Error(_) => {
            return HttpResponse::InternalServerError().body("Error getting courses");
        }
    };
    render_template!(handlebars, "config-test", json!({"title": "Start a new test", "courses": courses}))
}

#[get("/start-test/{test_id}")]
pub async fn test_start_page(
    handlebars: web::Data<Handlebars<'_>>,
    path: web::Path<u32>,
) -> impl Responder {
    let test_id = path.into_inner();
    render_template!(handlebars, "start-test", json!({"title": "Start a new test", "test_id":test_id}))
}
