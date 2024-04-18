use aarya_utils::hash_ops;
use actix_web::{delete, get, post, put, web, HttpResponse, Responder};
use models::{QuestionMutationModel, QuestionQueryModel};
use sqlx::MySqlPool;
use time::OffsetDateTime;
use validator::Validate;

use crate::entities::{questions::QuestionEntity, result_type::EntityResult};

#[post("/question")]
pub async fn question_create(pool: web::Data<MySqlPool>, model: web::Json<QuestionMutationModel>) -> impl Responder {
    let model = model.0;
    println!("{:?}", model);

    match model.validate() {
        Ok(_) => (),
        Err(e) => {
            println!("{:?}", e);
            return HttpResponse::BadRequest().body(format!("Validation error: [{:?}]", e));
        }
    }
    println!("transforming model to entity");
    let question = QuestionEntity {
        question_id: Some(model.question_id),
        course_id: model.course_id,
        chapter_id: model.chapter_id,
        id_hash: hash_ops::fast_hash(OffsetDateTime::now_utc().nanosecond().to_string().as_str()),
        que_text: model.que_text.clone(),
        que_description: model.que_description.clone(),
        choices: model.choices.clone(),
        answers: model.answers.clone(),
        ans_explanation: model.ans_explanation.clone(),
        ans_hint: model.ans_hint.clone(),
        difficulty: model.que_difficulty as i8,
        diff_reason: model.diff_reason.clone(),
        added_timestamp: None,
        updated_timestamp: None,
        que_hash: hash_ops::string_hasher(&model.que_text),
    };
    println!("{:?}", question);
    match question.create(&pool).await {
        EntityResult::Success(r) => HttpResponse::Ok().body(format!("Question created: [{:?}]", r)),
        EntityResult::Error(e) => HttpResponse::InternalServerError().body(format!("Failed to post question: [{:?}]", e)),
    }
}

#[get("/questions")]
pub async fn get_all_questions(pool: web::Data<MySqlPool>) -> impl Responder {
    let question = QuestionEntity::new();
    match question.read_all(&pool).await {
        EntityResult::Success(result) => {
            let mut questions = Vec::new();
            for row in result {
                questions.push(QuestionQueryModel {
                    question_id: row.question_id,
                    course_id: row.course_id,
                    chapter_id: row.chapter_id,
                    id_hash: row.id_hash.clone(),
                    que_text: row.que_text.clone(),
                    que_description: row.que_description.clone(),
                    choices: row.choices.clone(),
                    ans_explanation: row.ans_explanation.clone(),
                    ans_hint: row.ans_hint.clone(),
                    que_difficulty: row.difficulty as u8,
                    diff_reason: row.diff_reason.clone(),
                    course_name: row.course_name,
                    chapter_name: row.chapter_name,
                });
            }
            HttpResponse::Ok().json(questions)
        }
        EntityResult::Error(e) => HttpResponse::InternalServerError().body(format!("Failed to fetch questions: [{:?}]", e)),
    }
}

#[get("/question/id/{id}")]
pub async fn get_questions_by_id_hash(pool: web::Data<MySqlPool>, path: web::Path<String>) -> impl Responder {
    let id_hash = path.into_inner();
    let mut question = QuestionEntity::new();
    question.id_hash = id_hash;
    match question.read_by_hash(&pool).await {
        EntityResult::Success(result) => match result {
            Some(row) => {
                let question = QuestionQueryModel {
                    question_id: row.question_id,
                    course_id: row.course_id,
                    chapter_id: row.chapter_id,
                    id_hash: row.id_hash.clone(),
                    que_text: row.que_text.clone(),
                    que_description: row.que_description.clone(),
                    choices: row.choices.clone(),
                    ans_explanation: row.ans_explanation.clone(),
                    ans_hint: row.ans_hint.clone(),
                    que_difficulty: row.difficulty as u8,
                    diff_reason: row.diff_reason.clone(),
                    course_name: row.course_name,
                    chapter_name: row.chapter_name,
                };
                HttpResponse::Ok().json(question)
            }
            None => HttpResponse::NotFound().body("Question not found"),
        },
        EntityResult::Error(_) => HttpResponse::InternalServerError().body("Failed to fetch question"),
    }
}

#[get("/question/chapter/{chapter_id}/course/{course_id}")]
pub async fn get_questions_by_chapter_course(pool: web::Data<MySqlPool>, path: web::Path<(u32, u32)>) -> impl Responder {
    let (chapter_id, course_id) = path.into_inner();
    let mut question = QuestionEntity::new();
    question.chapter_id = chapter_id;
    question.course_id = course_id;
    match question.read_all_with_course_chapter(&pool).await {
        EntityResult::Success(q) => {
            let mut questions = Vec::new();
            for row in q {
                questions.push(QuestionQueryModel {
                    question_id: row.question_id,
                    course_id: row.course_id,
                    chapter_id: row.chapter_id,
                    id_hash: row.id_hash.clone(),
                    que_text: row.que_text.clone(),
                    que_description: row.que_description.clone(),
                    choices: row.choices.clone(),
                    ans_explanation: row.ans_explanation.clone(),
                    ans_hint: row.ans_hint.clone(),
                    que_difficulty: row.difficulty as u8,
                    diff_reason: row.diff_reason.clone(),
                    course_name: row.course_name,
                    chapter_name: row.chapter_name,
                });
            }
            HttpResponse::Ok().json(questions)
        }
        EntityResult::Error(_) => HttpResponse::InternalServerError().body("Failed to fetch questions"),
    }
}

#[get("/question/chapter/{chapter_id}")]
pub async fn get_questions_by_chapter(pool: web::Data<MySqlPool>, path: web::Path<u32>) -> impl Responder {
    let chapter_id = path.into_inner();
    let mut question = QuestionEntity::new();
    question.chapter_id = chapter_id;
    match question.read_all_with_course_chapter(&pool).await {
        EntityResult::Success(q) => {
            let mut questions = Vec::new();
            for row in q {
                questions.push(QuestionQueryModel {
                    question_id: row.question_id,
                    course_id: row.course_id,
                    chapter_id: row.chapter_id,
                    id_hash: row.id_hash.clone(),
                    que_text: row.que_text.clone(),
                    que_description: row.que_description.clone(),
                    choices: row.choices.clone(),
                    ans_explanation: row.ans_explanation.clone(),
                    ans_hint: row.ans_hint.clone(),
                    que_difficulty: row.difficulty as u8,
                    diff_reason: row.diff_reason.clone(),
                    course_name: row.course_name,
                    chapter_name: row.chapter_name,
                });
            }
            HttpResponse::Ok().json(questions)
        }
        EntityResult::Error(_) => HttpResponse::InternalServerError().body("Failed to fetch questions"),
    }
}

#[get("/question/course/{course_id}")]
pub async fn get_questions_by_course(pool: web::Data<MySqlPool>, path: web::Path<u32>) -> impl Responder {
    let course_id = path.into_inner();
    let mut question = QuestionEntity::new();
    question.course_id = course_id;
    match question.read_all_with_course(&pool).await {
        EntityResult::Success(q) => {
            let mut questions = Vec::new();
            for row in q {
                questions.push(QuestionQueryModel {
                    question_id: row.question_id,
                    course_id: row.course_id,
                    chapter_id: row.chapter_id,
                    id_hash: row.id_hash.clone(),
                    que_text: row.que_text.clone(),
                    que_description: row.que_description.clone(),
                    choices: row.choices.clone(),
                    ans_explanation: row.ans_explanation.clone(),
                    ans_hint: row.ans_hint.clone(),
                    que_difficulty: row.difficulty as u8,
                    diff_reason: row.diff_reason.clone(),
                    course_name: row.course_name,
                    chapter_name: row.chapter_name,
                });
            }
            HttpResponse::Ok().json(questions)
        }
        EntityResult::Error(_) => HttpResponse::InternalServerError().body("Failed to fetch questions"),
    }
}

#[get("/question/dedup/{que_hash}")]
pub async fn get_question_by_deduplicating_hash(pool: web::Data<MySqlPool>, path: web::Path<String>) -> impl Responder {
    let que_hash = path.into_inner();
    let mut question = QuestionEntity::new();
    question.que_hash = que_hash;
    match question.read_by_q_hash(&pool).await {
        EntityResult::Success(result) => match result {
            Some(row) => {
                let question = QuestionQueryModel {
                    question_id: row.question_id,
                    course_id: row.course_id,
                    chapter_id: row.chapter_id,
                    id_hash: row.id_hash.clone(),
                    que_text: row.que_text.clone(),
                    que_description: row.que_description.clone(),
                    choices: row.choices.clone(),
                    ans_explanation: row.ans_explanation.clone(),
                    ans_hint: row.ans_hint.clone(),
                    que_difficulty: row.difficulty as u8,
                    diff_reason: row.diff_reason.clone(),
                    course_name: row.course_name,
                    chapter_name: row.chapter_name,
                };
                HttpResponse::Ok().json(question)
            }
            None => HttpResponse::NotFound().body("Question not found"),
        },
        EntityResult::Error(_) => HttpResponse::InternalServerError().body("Failed to fetch question"),
    }
}

#[put("/question")]
pub async fn update_question_by_id(pool: web::Data<MySqlPool>, model: web::Json<QuestionMutationModel>) -> impl Responder {
    match model.validate() {
        Ok(_) => (),
        Err(e) => {
            return HttpResponse::BadRequest().body(e.to_string());
        }
    }
    let question = QuestionEntity {
        question_id: Some(model.question_id),
        course_id: model.course_id,
        chapter_id: model.chapter_id,
        id_hash: model.id_hash.clone(),
        que_text: model.que_text.clone(),
        que_description: model.que_description.clone(),
        choices: model.choices.clone(),
        answers: model.answers.clone(),
        ans_explanation: model.ans_explanation.clone(),
        ans_hint: model.ans_hint.clone(),
        difficulty: model.que_difficulty as i8,
        diff_reason: model.diff_reason.clone(),
        added_timestamp: None,
        updated_timestamp: None,
        que_hash: hash_ops::string_hasher(&model.que_text),
    };
    match question.update(&pool).await {
        EntityResult::Success(_) => HttpResponse::Ok().body("Question updated"),
        EntityResult::Error(_) => HttpResponse::InternalServerError().body("Failed to update question"),
    }
}

#[delete("/question")]
pub async fn delete_question_by_id(pool: web::Data<MySqlPool>, model: web::Json<QuestionQueryModel>) -> impl Responder {
    match model.validate() {
        Ok(_) => (),
        Err(e) => {
            return HttpResponse::BadRequest().body(e.to_string());
        }
    }

    let mut question = QuestionEntity::new();
    question.question_id = Some(model.question_id);

    match question.delete(&pool).await {
        EntityResult::Success(_) => HttpResponse::Ok().body("Question deleted"),
        EntityResult::Error(_) => HttpResponse::InternalServerError().body("Failed to delete question"),
    }
}
