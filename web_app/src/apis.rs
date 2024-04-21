use actix_web::{get, web, HttpResponse, Responder};
use models::{chapters::ChapterEntity, result_types::EntityResult, topics::TopicEntity};
use sqlx::MySqlPool;

// get chapters -> GET /chapters/{course_id}
#[get("/chapters/{course_id}")]
pub async fn chapters_by_course(pool: web::Data<MySqlPool>, path: web::Path<String>) -> impl Responder {
    let course_id = path.into_inner();
    let mut chapter = ChapterEntity::new();
    chapter.course_id = Some(course_id.parse().unwrap());

    match chapter.find_by_course(&pool).await {
        EntityResult::Success(chapters) => HttpResponse::Ok().json(chapters),
        EntityResult::Error(_) => HttpResponse::InternalServerError().body("Error getting chapters"),
    }
}

// get topics -> GET /topics/{chapter_id}
#[get("/topics/{chapter_id}/{course_id}")]
pub async fn topics_by_chapter(pool: web::Data<MySqlPool>, path: web::Path<(String, String)>) -> impl Responder {
    let (chapter_id, course_id) = path.into_inner();
    let mut topic = TopicEntity::new();
    topic.chapter_id = Some(chapter_id.parse().unwrap());
    topic.course_id = Some(course_id.parse().unwrap());

    match topic.find(&pool).await {
        EntityResult::Success(topics) => HttpResponse::Ok().json(topics),
        EntityResult::Error(_) => HttpResponse::InternalServerError().body("Error getting topics"),
    }
}

// post start-test -> POST /start-test
