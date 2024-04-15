use aarya_utils::{hash_ops, models::question_model::QuestionModel};
use actix_web::{delete, get, post, put, web, HttpResponse, Responder};
use sqlx::MySqlPool;
use validator::Validate;

use crate::entities::{error_type::EntityResult, question_entity::QuestionEntity};

#[post("/question")]
pub async fn question_create(pool: web::Data<MySqlPool>, model: web::Json<QuestionModel>) -> impl Responder {
    match model.validate() {
        Ok(_) => (),
        Err(e) => {
            return HttpResponse::BadRequest().body(e.to_string());
        }
    }
    let question = QuestionEntity {
        question_id: model.question_id,
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
    match question.create(&pool).await {
        EntityResult::Success(_) => HttpResponse::Ok().body("Question posted"),
        EntityResult::Error(_) => HttpResponse::InternalServerError().body("Failed to post question"),
    }
}

#[get("/questions")]
pub async fn get_all_questions(pool: web::Data<MySqlPool>) -> impl Responder {
    let question = QuestionEntity::new();
    match question.read_all(&pool).await {
        EntityResult::Success(result) => {
            let mut questions = Vec::new();
            for row in result {
                questions.push(QuestionModel {
                    question_id: row.question_id,
                    course_id: row.course_id,
                    chapter_id: row.chapter_id,
                    id_hash: row.id_hash.clone(),
                    que_text: row.que_text.clone(),
                    que_description: row.que_description.clone(),
                    choices: row.choices.clone(),
                    answers: row.answers.clone(),
                    ans_explanation: row.ans_explanation.clone(),
                    ans_hint: row.ans_hint.clone(),
                    que_difficulty: row.difficulty as u8,
                    diff_reason: row.diff_reason.clone(),
                });
            }
            HttpResponse::Ok().json(questions)
        }
        EntityResult::Error(_) => HttpResponse::InternalServerError().body("Failed to fetch questions"),
    }
}

#[get("/question/:id")]
pub async fn get_questions_by_id_hash(pool: web::Data<MySqlPool>, path: web::Path<String>) -> impl Responder {
    let id_hash = path.into_inner();
    let mut question = QuestionEntity::new();
    question.id_hash = id_hash;
    match question.read_by_hash(&pool).await {
        EntityResult::Success(result) => match result {
            Some(row) => {
                let question = QuestionModel {
                    question_id: row.question_id,
                    course_id: row.course_id,
                    chapter_id: row.chapter_id,
                    id_hash: row.id_hash.clone(),
                    que_text: row.que_text.clone(),
                    que_description: row.que_description.clone(),
                    choices: row.choices.clone(),
                    answers: row.answers.clone(),
                    ans_explanation: row.ans_explanation.clone(),
                    ans_hint: row.ans_hint.clone(),
                    que_difficulty: row.difficulty as u8,
                    diff_reason: row.diff_reason.clone(),
                };
                HttpResponse::Ok().json(question)
            }
            None => HttpResponse::NotFound().body("Question not found"),
        },
        EntityResult::Error(_) => HttpResponse::InternalServerError().body("Failed to fetch question"),
    }
}

#[get("/question/:chapter_id/:course_id")]
pub async fn get_questions_by_chapter_course(pool: web::Data<MySqlPool>, path: web::Path<(u32, u32)>) -> impl Responder {
    let (chapter_id, course_id) = path.into_inner();
    let mut question = QuestionEntity::new();
    question.chapter_id = chapter_id;
    question.course_id = course_id;
    match question.read_by_chapter_course(&pool).await {
        EntityResult::Success(q) => {
            let mut questions = Vec::new();
            for row in q {
                questions.push(QuestionModel {
                    question_id: row.question_id,
                    course_id: row.course_id,
                    chapter_id: row.chapter_id,
                    id_hash: row.id_hash.clone(),
                    que_text: row.que_text.clone(),
                    que_description: row.que_description.clone(),
                    choices: row.choices.clone(),
                    answers: row.answers.clone(),
                    ans_explanation: row.ans_explanation.clone(),
                    ans_hint: row.ans_hint.clone(),
                    que_difficulty: row.difficulty as u8,
                    diff_reason: row.diff_reason.clone(),
                });
            }
            HttpResponse::Ok().json(questions)
        }
        EntityResult::Error(_) => HttpResponse::InternalServerError().body("Failed to fetch questions"),
    }
}

#[get("/question/:chapter_id")]
pub async fn get_questions_by_chapter(pool: web::Data<MySqlPool>, path: web::Path<u32>) -> impl Responder {
    let chapter_id = path.into_inner();
    let mut question = QuestionEntity::new();
    question.chapter_id = chapter_id;
    match question.read_by_chapter(&pool).await {
        EntityResult::Success(q) => {
            let mut questions = Vec::new();
            for row in q {
                questions.push(QuestionModel {
                    question_id: row.question_id,
                    course_id: row.course_id,
                    chapter_id: row.chapter_id,
                    id_hash: row.id_hash.clone(),
                    que_text: row.que_text.clone(),
                    que_description: row.que_description.clone(),
                    choices: row.choices.clone(),
                    answers: row.answers.clone(),
                    ans_explanation: row.ans_explanation.clone(),
                    ans_hint: row.ans_hint.clone(),
                    que_difficulty: row.difficulty as u8,
                    diff_reason: row.diff_reason.clone(),
                });
            }
            HttpResponse::Ok().json(questions)
        }
        EntityResult::Error(_) => HttpResponse::InternalServerError().body("Failed to fetch questions"),
    }
}

#[get("/question/:course_id")]
pub async fn get_questions_by_course(pool: web::Data<MySqlPool>, path: web::Path<u32>) -> impl Responder {
    let course_id = path.into_inner();
    let mut question = QuestionEntity::new();
    question.course_id = course_id;
    match question.read_by_course(&pool).await {
        EntityResult::Success(q) => {
            let mut questions = Vec::new();
            for row in q {
                questions.push(QuestionModel {
                    question_id: row.question_id,
                    course_id: row.course_id,
                    chapter_id: row.chapter_id,
                    id_hash: row.id_hash.clone(),
                    que_text: row.que_text.clone(),
                    que_description: row.que_description.clone(),
                    choices: row.choices.clone(),
                    answers: row.answers.clone(),
                    ans_explanation: row.ans_explanation.clone(),
                    ans_hint: row.ans_hint.clone(),
                    que_difficulty: row.difficulty as u8,
                    diff_reason: row.diff_reason.clone(),
                });
            }
            HttpResponse::Ok().json(questions)
        }
        EntityResult::Error(_) => HttpResponse::InternalServerError().body("Failed to fetch questions"),
    }
}

#[get("/question/:deduplicating_hash")]
pub async fn get_question_by_deduplicating_hash(pool: web::Data<MySqlPool>, path: web::Path<String>) -> impl Responder {
    let deduplicating_hash = path.into_inner();
    let mut question = QuestionEntity::new();
    question.que_hash = deduplicating_hash;
    match question.read_by_q_hash(&pool).await {
        EntityResult::Success(result) => match result {
            Some(row) => {
                let question = QuestionModel {
                    question_id: row.question_id,
                    course_id: row.course_id,
                    chapter_id: row.chapter_id,
                    id_hash: row.id_hash.clone(),
                    que_text: row.que_text.clone(),
                    que_description: row.que_description.clone(),
                    choices: row.choices.clone(),
                    answers: row.answers.clone(),
                    ans_explanation: row.ans_explanation.clone(),
                    ans_hint: row.ans_hint.clone(),
                    que_difficulty: row.difficulty as u8,
                    diff_reason: row.diff_reason.clone(),
                };
                HttpResponse::Ok().json(question)
            }
            None => HttpResponse::NotFound().body("Question not found"),
        },
        EntityResult::Error(_) => HttpResponse::InternalServerError().body("Failed to fetch question"),
    }
}

#[put("/question")]
pub async fn update_question_by_id(pool: web::Data<MySqlPool>, model: web::Json<QuestionModel>) -> impl Responder {
    match model.validate() {
        Ok(_) => (),
        Err(e) => {
            return HttpResponse::BadRequest().body(e.to_string());
        }
    }
    let question = QuestionEntity {
        question_id: model.question_id,
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
pub async fn delete_question_by_id(pool: web::Data<MySqlPool>, model: web::Json<QuestionModel>) -> impl Responder {
    match model.validate() {
        Ok(_) => (),
        Err(e) => {
            return HttpResponse::BadRequest().body(e.to_string());
        }
    }

    let mut question = QuestionEntity::new();
    question.question_id = model.question_id;

    match question.delete(&pool).await {
        EntityResult::Success(_) => HttpResponse::Ok().body("Question deleted"),
        EntityResult::Error(_) => {
            return HttpResponse::InternalServerError().body("Failed to delete question");
        }
    }
}
