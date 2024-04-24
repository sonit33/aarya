use actix_web::{get, post, web, HttpResponse, Responder};
use models::{
    chapters::ChapterEntity,
    questions::QuestionEntity,
    result_types::{EntityResult, SuccessResultType},
    tests::{TestEntity, TestMutationModel, TestQuestionsEntity},
    topics::TopicEntity,
};
use sqlx::MySqlPool;

// get chapters -> GET /chapters/{course_id}
#[get("/api/chapters/{course_id}")]
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
#[get("/api/topics/{chapter_id}/{course_id}")]
pub async fn topics_by(pool: web::Data<MySqlPool>, path: web::Path<(String, String)>) -> impl Responder {
    let (chapter_id, course_id) = path.into_inner();
    let mut topic = TopicEntity::new();
    topic.chapter_id = Some(chapter_id.parse().unwrap());
    topic.course_id = Some(course_id.parse().unwrap());

    match topic.find(&pool).await {
        EntityResult::Success(topics) => HttpResponse::Ok().json(topics),
        EntityResult::Error(e) => HttpResponse::InternalServerError().body(format!("Error getting topics: [{e:?}]")),
    }
}

/// depending on the context (course or chapter) questions are loaded dynamically
/// test variables are stored in the tests table (student_id, test_id, course_id, chapter_id, topic_id, difficulty, length, state)
/// state: incomplete (default, 0), complete (1)
/// find matching questions
/// save the matching questions in test_questions table (test_id, question_id, state)
/// state: unseen (default, 0), seen (1), answered (2)
#[post("/api/start-test")]
pub async fn start_test(pool: web::Data<MySqlPool>, model: web::Json<TestMutationModel>) -> impl Responder {
    let model = model.into_inner();
    let test = TestEntity {
        test_id: Some(0),
        // TODO: hard coded; replace with actual student id
        student_id: 10000,
        course_id: model.course_id,
        chapter_id: model.chapter_id,
        topic_id: model.topic_id,
        test_difficulty: model.test_difficulty,
        test_length: model.clone().test_length,
        test_state: model.test_state,
    };

    let pool = pool.clone();

    let test_id = match test.create_test(&pool).await {
        EntityResult::Success(r) => match r {
            SuccessResultType::Created(id, _) => id,
            _ => 0,
        },
        EntityResult::Error(e) => {
            return HttpResponse::InternalServerError().body(format!("Error creating a new test: [{e:?}]"));
        }
    };

    let model = model.clone();

    // find questions based on the test parameters
    let mut question = QuestionEntity::new();
    question.difficulty = model.test_difficulty as i8;
    question.chapter_id = model.chapter_id;
    question.topic_id = model.topic_id;
    question.course_id = model.course_id;

    match question.find_top_n(&pool, model.test_length).await {
        EntityResult::Success(questions) => {
            for question in questions {
                let test_question = TestQuestionsEntity {
                    test_id: test_id as u32,
                    question_id: question.question_id,
                    state: 0,
                };
                match test_question.create(&pool).await {
                    EntityResult::Success(_) => {}
                    EntityResult::Error(e) => {
                        return HttpResponse::InternalServerError().body(format!("Error saving questions: [{e:?}]"));
                    }
                }
            }
        }
        EntityResult::Error(e) => {
            return HttpResponse::InternalServerError().body(format!("Error getting questions: [{e:?}]"));
        }
    }

    HttpResponse::Ok().json(test_id)
}

// post start-test -> POST /start-test
