use actix_web::{get, web, HttpResponse, Responder};
use models::ChapterQueryModel;
use sqlx::MySqlPool;

use crate::entities::{chapters::ChapterEntity, result_type::EntityResult};

#[get("/api/chapters/course/{id_hash}")]
pub async fn get_all_courses(pool: web::Data<MySqlPool>, path: web::Path<String>) -> impl Responder {
    let id_hash = path.into_inner();
    let chapter = ChapterEntity::default();
    match chapter.get_chapters_by_course(&pool, id_hash).await {
        EntityResult::Success(entities) => {
            let mut result: Vec<ChapterQueryModel> = Vec::new();
            for entity in entities {
                result.push(ChapterQueryModel {
                    chapter_id: entity.chapter_id,
                    id_hash: entity.id_hash,
                    name: entity.name,
                    description: entity.description,
                    course_name: entity.course_name,
                    course_id_hash: entity.course_id_hash,
                });
            }
            HttpResponse::Ok().json(result)
        }
        EntityResult::Error(e) => HttpResponse::InternalServerError().body(format!("Failed to fetch chapters: [{:?}]", e)),
    }
}
