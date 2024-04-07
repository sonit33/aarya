use serde_json::Value;
use sqlx::mysql::MySqlQueryResult;
use sqlx::{Error, MySqlPool};
use time::OffsetDateTime;

#[derive(Debug, sqlx::FromRow)]
pub struct Question {
    question_id: u32,
    course_id: u32,
    chapter_id: u32,
    id_hash: String,
    q_text: String,
    choices: Value, // Assuming JSON structure is [{ "id": "", "text": "" }]
    answers: Value, // Assuming JSON structure is [{ "id": "" }]
    a_explanation: String,
    a_hint: String,
    q_difficulty: i8,
    diff_reason: String,
    added_timestamp: Option<OffsetDateTime>,
    updated_timestamp: Option<OffsetDateTime>,
}

impl Question {
    pub async fn create(
        pool: &MySqlPool,
        course_id: i32,
        question: &str,
        answers: &str,
        choices: Option<i32>,
        q_difficulty: Option<i32>,
        d_reason: Option<&str>,
        a_explanation: Option<&str>,
        a_hint: Option<&str>,
        q_mode: i8,
    ) -> Result<MySqlQueryResult, Error> {
        let res = sqlx
            ::query(
                "INSERT INTO questions (course_id, question, answers, choices, q_difficulty, d_reason, a_explanation, a_hint, q_mode) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)"
            )
            .bind(course_id)
            .bind(question)
            .bind(answers)
            .bind(choices)
            .bind(q_difficulty)
            .bind(d_reason)
            .bind(a_explanation)
            .bind(a_hint)
            .bind(q_mode)
            .execute(pool).await;
        match res {
            Ok(result) => Ok(result),
            Err(e) => Err(e),
        }
    }

    pub async fn read(pool: &MySqlPool, question_id: i32) -> Result<Option<Question>, Error> {
        let question =
            sqlx::query_as::<_, Question>("SELECT * FROM questions WHERE question_id = ?")
                .bind(question_id)
                .fetch_optional(pool)
                .await;
        match question {
            Ok(result) => Ok(result),
            Err(e) => Err(e),
        }
    }

    pub async fn update(
        pool: &MySqlPool,
        question_id: i32,
        course_id: i32,
        question: &str,
        answers: &str,
        choices: Option<i32>,
        q_difficulty: Option<i32>,
        d_reason: Option<&str>,
        a_explanation: Option<&str>,
        a_hint: Option<&str>,
        q_mode: i8,
    ) -> Result<MySqlQueryResult, Error> {
        let res = sqlx
            ::query(
                "UPDATE questions SET course_id = ?, question = ?, answers = ?, choices = ?, q_difficulty = ?, d_reason = ?, a_explanation = ?, a_hint = ?, q_mode = ? WHERE question_id = ?"
            )
            .bind(course_id)
            .bind(question)
            .bind(answers)
            .bind(choices)
            .bind(q_difficulty)
            .bind(d_reason)
            .bind(a_explanation)
            .bind(a_hint)
            .bind(q_mode)
            .bind(question_id)
            .execute(pool).await;
        match res {
            Ok(result) => Ok(result),
            Err(e) => Err(e),
        }
    }

    pub async fn delete(pool: &MySqlPool, question_id: i32) -> Result<MySqlQueryResult, Error> {
        let res = sqlx::query("DELETE FROM questions WHERE question_id = ?")
            .bind(question_id)
            .execute(pool)
            .await;
        match res {
            Ok(result) => Ok(result),
            Err(e) => Err(e),
        }
    }
}
