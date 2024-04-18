use actix_web::{get, web, HttpResponse, Responder};
use models::CourseQueryModel;
use sqlx::MySqlPool;

use crate::entities::{courses::CourseEntity, result_type::EntityResult};

#[get("/chapters/{course_id}")]
pub async fn get_all_courses(pool: web::Data<MySqlPool>) -> impl Responder {
    let course = CourseEntity::default();
    course.course_id = Some(course_id);
    match course.get_chapters_with_course(&pool).await {
        EntityResult::Success(entities) => {
            let mut result: Vec<ChapterWithCourse> = Vec::new();
            for entity in entities {
                result.push(ChapterWithCourse {
                    chapter_id: entity.chapter_id,
                    id_hash: entity.id_hash,
                    course_id: entity.course_id,
                    course_name: entity.course_name,
                    chapter_name: entity.chapter_name,
                    description: entity.description,
                });
            }
            HttpResponse::Ok().json(result)
        }
        EntityResult::Error(e) => HttpResponse::InternalServerError().body(format!("Failed to fetch chapters: [{:?}]", e)),
    }
}
