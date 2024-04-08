use aarya_utils::hasher;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use sqlx::mysql::MySqlQueryResult;
use sqlx::{Error, MySqlPool};
use time::OffsetDateTime;

#[derive(Debug, Serialize, Deserialize)]
pub struct Choice {
    id: String,
    text: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Answer {
    id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QuestionFromJson {
    pub q_text: String,
    pub choices: Vec<Choice>,
    pub answers: Vec<Answer>,
    pub a_explanation: String,
    pub a_hint: String,
    pub difficulty: i8,
    pub diff_reason: String,
}

#[derive(Debug, sqlx::FromRow)]
pub struct Question {
    pub question_id: Option<u32>,
    pub course_id: u32,
    pub chapter_id: u32,
    pub id_hash: String,
    pub q_text: String,
    pub choices: Value, // Assuming JSON structure is [{ "id": "", "text": "" }]
    pub answers: Value, // Assuming JSON structure is [{ "id": "" }]
    pub a_explanation: String,
    pub a_hint: String,
    pub difficulty: i8,
    pub diff_reason: String,
    pub added_timestamp: Option<OffsetDateTime>,
    pub updated_timestamp: Option<OffsetDateTime>,
    pub q_hash: String,
}

impl Question {
    pub fn new() -> Self {
        Question {
            question_id: Some(0),
            course_id: 0,
            chapter_id: 0,
            id_hash: "not-set".to_string(),
            q_text: "not-set".to_string(),
            choices: json!([{"id":"", "text":"not-set"}]),
            answers: json!([{"id":""}]),
            a_explanation: "not-set".to_string(),
            a_hint: "not-set".to_string(),
            difficulty: 0,
            diff_reason: "not-set".to_string(),
            added_timestamp: None,
            updated_timestamp: None,
            q_hash: String::from("random"),
        }
    }
    pub fn random(hash: &String) -> Self {
        Question {
            question_id: Some(0),
            course_id: 0,
            chapter_id: 0,
            id_hash: hash.to_string(),
            q_text: hash.to_string(),
            choices: json!([{"id":"", "text":hash.to_string()}]),
            answers: json!([{"id":""}]),
            a_explanation: hash.to_string(),
            a_hint: hash.to_string(),
            difficulty: 0,
            diff_reason: hash.to_string(),
            added_timestamp: None,
            updated_timestamp: None,
            q_hash: String::from("random"),
        }
    }
}

impl Question {
    pub async fn create(&self, pool: &MySqlPool) -> Result<MySqlQueryResult, Error> {
        let q_hash = hasher::cook_hash(&self.q_text.to_lowercase().as_str()).unwrap();
        let res = sqlx
            ::query(
                "INSERT INTO questions (course_id, chapter_id, id_hash, q_text, answers, choices, difficulty, diff_reason, a_explanation, a_hint, q_hash) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"
            )
            .bind(&self.course_id)
            .bind(&self.chapter_id)
            .bind(&self.id_hash)
            .bind(&self.q_text)
            .bind(&self.answers)
            .bind(&self.choices)
            .bind(&self.difficulty)
            .bind(&self.diff_reason)
            .bind(&self.a_explanation)
            .bind(&self.a_hint)
            .bind(q_hash)
            .execute(pool).await;
        match res {
            Ok(result) => Ok(result),
            Err(e) => Err(e),
        }
    }

    pub async fn read(&self, pool: &MySqlPool) -> Result<Option<Question>, Error> {
        let question =
            sqlx::query_as::<_, Question>("SELECT * FROM questions WHERE question_id = ?")
                .bind(&self.question_id)
                .fetch_optional(pool)
                .await;
        match question {
            Ok(result) => Ok(result),
            Err(e) => Err(e),
        }
    }

    pub async fn read_by_chapter(&self, pool: &MySqlPool) -> Result<Vec<Question>, Error> {
        let question =
            sqlx::query_as::<_, Question>("SELECT * FROM questions WHERE chapter_id = ?")
                .bind(&self.chapter_id)
                .fetch_all(pool)
                .await;
        match question {
            Ok(result) => Ok(result),
            Err(e) => Err(e),
        }
    }

    pub async fn read_by_course(&self, pool: &MySqlPool) -> Result<Vec<Question>, Error> {
        let question = sqlx::query_as::<_, Question>("SELECT * FROM questions WHERE course_id = ?")
            .bind(&self.course_id)
            .fetch_all(pool)
            .await;
        match question {
            Ok(result) => Ok(result),
            Err(e) => Err(e),
        }
    }

    pub async fn read_by_hash(&self, pool: &MySqlPool) -> Result<Option<Question>, Error> {
        let question = sqlx::query_as::<_, Question>("SELECT * FROM questions WHERE id_hash = ?")
            .bind(&self.id_hash)
            .fetch_optional(pool)
            .await;
        match question {
            Ok(result) => Ok(result),
            Err(e) => Err(e),
        }
    }

    pub async fn update(&self, pool: &MySqlPool) -> Result<MySqlQueryResult, Error> {
        let res = sqlx
            ::query(
                "UPDATE questions SET course_id = ?, chapter_id = ?, id_hash = ?, q_text = ?, answers = ?, choices = ?, difficulty = ?, diff_reason = ?, a_explanation = ?, a_hint = ? WHERE question_id = ?"
            )
            .bind(&self.course_id)
            .bind(&self.chapter_id)
            .bind(&self.id_hash)
            .bind(&self.q_text)
            .bind(&self.answers)
            .bind(&self.choices)
            .bind(&self.difficulty)
            .bind(&self.diff_reason)
            .bind(&self.a_explanation)
            .bind(&self.a_hint)
            .bind(&self.question_id)
            .execute(pool).await;
        match res {
            Ok(result) => Ok(result),
            Err(e) => Err(e),
        }
    }

    pub async fn delete(&self, pool: &MySqlPool) -> Result<MySqlQueryResult, Error> {
        let res = sqlx::query("DELETE FROM questions WHERE question_id = ?")
            .bind(&self.question_id)
            .execute(pool)
            .await;
        match res {
            Ok(result) => Ok(result),
            Err(e) => Err(e),
        }
    }
}
