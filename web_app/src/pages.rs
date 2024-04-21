use actix_web::{get, web, HttpResponse, Responder};
use handlebars::Handlebars;
use models::{courses::CourseEntity, result_types::EntityResult};
use serde_json::json;
use sqlx::MySqlPool;

#[get("/")]
pub async fn home_page(handlebars: web::Data<Handlebars<'_>>) -> impl Responder {
    render_template!(handlebars, "index", json!({"title": "Aarya welcomes you!"}))
}

// What do you need to start a test?
// - courses -> GET /courses
// - chapters -> GET /chapters/{course_id}
// - topics -> GET /topics/{chapter_id}

// What happens when you click on "start test"?
// - POST /start-test
//     - depending on the context (course or chapter) questions are loaded dynamically
//         - test variables are stored in the tests table (student_id, test_id, course_id, chapter_id, topic_id, difficulty, length, state)
//             - state: incomplete (default, 0), complete (1)
//         - find matching questions
//         - save the matching questions in test_questions table (test_id, question_id, state)
//             - state: unseen (default, 0), seen (1), answered (2)
// - if OK then enter the test mode

// What is a test mode?
// You see one question at a time. You see the next question after you submit.

// How do I see the first question?
// - GET /test/{test_id}/next
//     - select top 1 from test_questions where state == `unseen`

// How do I submit my answer?
// - POST /test/{test_id}/{question_id}
//     - mark state == 'answered'
//     - if OK then GET /test/{test_id}/next

// What happends when you submit a test?
// - POST /test/submit/{test_id}

// Test taking experience (UI):
// navigation circles (1) (2) (3)              button: exit
// pills: que_difficulty (Level 1, etc.), course_name, chapter_name
// subtext: diff_reason
// p: (1) que_text
// p: (radio or checkbox) choices
// button: <- back (left aligned) submit -> (right aligned)
///
#[get("/start-test")]
pub async fn start_test_page(handlebars: web::Data<Handlebars<'_>>, pool: web::Data<MySqlPool>) -> impl Responder {
    let course = CourseEntity::new();
    let courses = match course.find_all(&pool).await {
        EntityResult::Success(courses) => courses,
        EntityResult::Error(_) => {
            return HttpResponse::InternalServerError().body("Error getting courses");
        }
    };
    render_template!(handlebars, "start-test", json!({"title": "Start a new test", "courses": courses}))
}
