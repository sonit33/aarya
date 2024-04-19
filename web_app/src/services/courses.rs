use actix_web::{get, web, HttpResponse, Responder};
use models::CourseQueryModel;
use sqlx::MySqlPool;

use crate::entities::{courses::CourseEntity, result_type::EntityResult};

#[get("/api/courses")]
pub async fn get_all_courses(pool: web::Data<MySqlPool>) -> impl Responder {
    let course = CourseEntity::default();
    match course.read_all(&pool).await {
        EntityResult::Success(entities) => {
            let mut result: Vec<CourseQueryModel> = Vec::new();
            for entity in entities {
                result.push(CourseQueryModel {
                    name: entity.name,
                    id_hash: entity.id_hash,
                    description: entity.description,
                });
            }
            HttpResponse::Ok().json(result)
        }
        EntityResult::Error(e) => HttpResponse::InternalServerError().body(format!("Failed to fetch courses: [{:?}]", e)),
    }
}
