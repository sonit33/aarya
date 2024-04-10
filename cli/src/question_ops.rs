use aarya_models::database::question::{Question, QuestionFromJson};
use aarya_utils::{db_ops::setup_durable_database, environ::Environ, hasher, random::generate_guid};
use serde_json::json;
use sqlx::{MySql, Pool};

use std::future::Future;

pub async fn save_with<F, Fut>(questions: Vec<QuestionFromJson>, setup_db: F)
where
    F: FnOnce(String) -> Fut,
    Fut: Future<Output = Result<Pool<MySql>, sqlx::Error>>,
{
    let env_default = Environ::default();
    match setup_db(env_default.db_connection_string).await {
        Ok(pool) => {
            for question in questions {
                save_question(&pool, question).await;
            }
        }
        Err(e) => {
            println!("Failed to established database connection: {}", e)
        }
    }
}

// extract database and environ to make it testable
pub async fn save(questions: Vec<QuestionFromJson>) {
    let env_default = Environ::default();
    match setup_durable_database(format!("{}/{}", env_default.db_connection_string, env_default.db_name)).await {
        Ok(pool) => {
            for question in questions {
                save_question(&pool, question).await;
            }
        }
        Err(e) => {
            println!("Failed to established database connection: {}", e)
        }
    }
}

async fn save_question(pool: &Pool<MySql>, question: QuestionFromJson) {
    let mut q = Question::new();
    q.course_id = 2;
    q.chapter_id = 2;
    q.id_hash = hasher::fast_hash(generate_guid(8).as_str());
    q.q_text = question.q_text.to_string();
    q.choices = json!(question.choices);
    q.answers = json!(question.answers);
    q.a_explanation = question.a_explanation;
    q.a_hint = question.a_hint;
    q.difficulty = question.difficulty;
    q.diff_reason = question.diff_reason;
    match q.create_if(&pool).await {
        Ok(q) => match q {
            Some(q) => {
                println!("Question created: {:?}", q);
            }
            None => {
                println!("Question already exists");
            }
        },
        Err(e) => {
            println!("Failed to create question: [{}]", e);
        }
    }
}
