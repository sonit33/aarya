use actix_web::{get, web, HttpResponse, Responder};
use handlebars::Handlebars;
use models::{
    entities::{ChapterEntity, CourseEntity, EntityResult},
    models::{ChapterQueryModel, CourseQueryModel},
};
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
            let mut result: Vec<CourseQueryModel> = Vec::new();
            for entity in entities {
                result.push(CourseQueryModel {
                    name: entity.name,
                    id_hash: entity.id_hash,
                    description: entity.description,
                });
            }
            // Render the courses template using Handlebars
            match handlebars.render("courses", &json!({"courses": result})) {
                Ok(body) => HttpResponse::Ok().body(body),
                Err(e) => {
                    println!("Error rendering courses template: {:?}", e);

                    HttpResponse::InternalServerError().finish()
                }
            }
        }
        EntityResult::Error(e) => HttpResponse::InternalServerError().body(format!("Failed to fetch courses: [{:?}]", e)),
    }
}

#[get("/courses/{id_hash}/chapters")]
/// Get all chapters for a course accept an id_hash parameter
/// the chapters are in the Chapter entity and its output must be in the ChapterQueryModel
pub async fn chapters_page(handlebars: web::Data<Handlebars<'_>>, pool: web::Data<MySqlPool>, id_hash: web::Path<String>) -> impl Responder {
    let chapter = ChapterEntity::default();
    let id_hash = id_hash.into_inner();
    match chapter.find_by_course(&pool, id_hash).await {
        EntityResult::Success(entities) => {
            let mut result: Vec<ChapterQueryModel> = Vec::new();
            for entity in entities {
                result.push(ChapterQueryModel {
                    name: entity.name,
                    id_hash: entity.id_hash,
                    description: entity.description,
                    course_id_hash: entity.course_id_hash,
                    course_name: entity.course_name,
                });
            }
            // Render the courses template using Handlebars
            match handlebars.render("chapters", &json!({"chapters": result})) {
                Ok(body) => HttpResponse::Ok().body(body),
                Err(e) => {
                    println!("Error rendering chapters template: {:?}", e);

                    HttpResponse::InternalServerError().finish()
                }
            }
        }
        EntityResult::Error(e) => HttpResponse::InternalServerError().body(format!("Failed to fetch chapters: [{:?}]", e)),
    }
}

#[get("/chapter/{id_hash}/tests")]
pub async fn tests_page() -> impl Responder {
    HttpResponse::Ok().body("Tests page")
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
