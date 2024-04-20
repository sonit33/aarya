use actix_web::{get, web, HttpResponse, Responder};
use handlebars::Handlebars;
use models::{chapters::ChapterEntity, courses::CourseEntity, result_types::EntityResult, tests::TestEntity};
use serde_json::json;
use sqlx::MySqlPool;

#[get("/")]
pub async fn home_page(handlebars: web::Data<Handlebars<'_>>) -> impl Responder {
    // Render the index template using Handlebars
    match handlebars.render("index", &json!({"title": "Aarya welcomes you!"})) {
        Ok(body) => HttpResponse::Ok().body(body),
        Err(e) => {
            println!("Error rendering index template: {:?}", e);

            HttpResponse::InternalServerError().finish()
        }
    }
}

#[get("/courses")]
pub async fn courses_page(handlebars: web::Data<Handlebars<'_>>, pool: web::Data<MySqlPool>) -> impl Responder {
    let course = CourseEntity::default();
    match course.find_all(&pool).await {
        EntityResult::Success(entities) => {
            render_template!(handlebars, "courses", json!({"courses": entities}))
        }
        EntityResult::Error(e) => HttpResponse::InternalServerError().body(format!("Failed to fetch courses: [{:?}]", e)),
    }
}

#[get("/chapters/{course_hash}")]
/// Get all chapters for a course accept an id_hash parameter
/// the chapters are in the Chapter entity and its output must be in the ChapterQueryModel
pub async fn chapters_page(handlebars: web::Data<Handlebars<'_>>, pool: web::Data<MySqlPool>, course_hash: web::Path<String>) -> impl Responder {
    let chapter = ChapterEntity::default();
    let id_hash = course_hash.into_inner();
    match chapter.find_by_course(&pool, id_hash).await {
        EntityResult::Success(entities) => {
            render_template!(handlebars, "chapters", json!({"chapters": entities}))
        }
        EntityResult::Error(e) => HttpResponse::InternalServerError().body(format!("Failed to fetch chapters: [{:?}]", e)),
    }
}

#[get("/tests/{chapter_hash}")]
pub async fn tests_page(handlebars: web::Data<Handlebars<'_>>, pool: web::Data<MySqlPool>, course_hash: web::Path<String>) -> impl Responder {
    let test = TestEntity::default();
    let id_hash = course_hash.into_inner();
    match test.find_by_chapter(&pool, id_hash).await {
        EntityResult::Success(entities) => {
            render_template!(handlebars, "tests", json!({"tests": entities}))
        }
        EntityResult::Error(e) => HttpResponse::InternalServerError().body(format!("Failed to fetch chapters: [{:?}]", e)),
    }
}

// /**
//  * /auth/signup
//  * /auth/login
//  * /auth/logout
//  * /auth/forgot-password
//  * /auth/reset-password
//  * /auth/verify-email
//  * /home
//  * /profile
//  * /courses --> get all courses
//  * /courses/{id_hash}/chapters --> get all chapters for a course
//  * /course/{id_hash}/tests --> get all tests for a course
//  * /chapter/{id_hash}/tests --> get all tests for a chapter
//  * /question/{id_hash} -->
//  * /tests --> get all tests
//  * /questions
//  */
