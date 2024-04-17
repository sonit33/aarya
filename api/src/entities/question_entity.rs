use aarya_utils::hash_ops;
use serde::{ Deserialize, Serialize };
use serde_json::{ json, Value };
use sqlx::MySqlPool;
use time::OffsetDateTime;

use super::result_type::{ DatabaseErrorType, EntityResult, SuccessResultType };

#[derive(Debug, Serialize, Deserialize)]
pub struct Choice {
    id: String,
    text: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Answer {
    id: String,
}
#[derive(Debug, sqlx::FromRow)]
pub struct QuestionEntity {
    pub question_id: u32,
    pub course_id: u32,
    pub chapter_id: u32,
    pub id_hash: String,
    pub que_text: String,
    pub que_description: String,
    pub choices: Value, // Assuming JSON structure is [{ "id": "", "text": "" }]
    pub answers: Value, // Assuming JSON structure is [{ "id": "" }]
    pub ans_explanation: String,
    pub ans_hint: String,
    pub difficulty: i8,
    pub diff_reason: String,
    pub added_timestamp: Option<OffsetDateTime>,
    pub updated_timestamp: Option<OffsetDateTime>,
    pub que_hash: String,
}

impl QuestionEntity {
    pub fn new() -> Self {
        QuestionEntity {
            question_id: 0,
            course_id: 0,
            chapter_id: 0,
            id_hash: "not-set".to_string(),
            que_text: "not-set".to_string(),
            que_description: "not-set".to_string(),
            choices: json!([{"id":"", "text":"not-set"}]),
            answers: json!([{"id":""}]),
            ans_explanation: "not-set".to_string(),
            ans_hint: "not-set".to_string(),
            difficulty: 0,
            diff_reason: "not-set".to_string(),
            added_timestamp: None,
            updated_timestamp: None,
            que_hash: String::from("random"),
        }
    }
}

impl QuestionEntity {
    pub async fn create(&self, pool: &MySqlPool) -> EntityResult<SuccessResultType> {
        let que_hash = hash_ops::string_hasher(self.que_text.to_lowercase().as_str());
        let added_timestamp = OffsetDateTime::now_utc();
        let res = sqlx
            ::query(
                "INSERT INTO questions (course_id, chapter_id, id_hash, que_text, que_description, answers, choices, difficulty, diff_reason, ans_explanation, ans_hint, que_hash, added_timestamp) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"
            )
            .bind(&self.course_id)
            .bind(&self.chapter_id)
            .bind(&self.id_hash)
            .bind(&self.que_text)
            .bind(&self.que_description)
            .bind(&self.answers)
            .bind(&self.choices)
            .bind(&self.difficulty)
            .bind(&self.diff_reason)
            .bind(&self.ans_explanation)
            .bind(&self.ans_hint)
            .bind(que_hash)
            .bind(added_timestamp)
            .execute(pool).await;
        match res {
            Ok(result) =>
                EntityResult::Success(
                    SuccessResultType::Created(result.last_insert_id(), result.rows_affected())
                ),
            Err(e) =>
                EntityResult::Error(
                    DatabaseErrorType::QueryError(
                        "Failed to create question".to_string(),
                        e.to_string()
                    )
                ),
        }
    }

    pub async fn read_all(&self, pool: &MySqlPool) -> EntityResult<Vec<QuestionEntity>> {
        let questions = sqlx
            ::query_as::<_, QuestionEntity>("SELECT * FROM questions")
            .fetch_all(pool).await;
        match questions {
            Ok(result) => EntityResult::Success(result),
            Err(e) =>
                EntityResult::Error(
                    DatabaseErrorType::QueryError(
                        "Failed to read questions".to_string(),
                        e.to_string()
                    )
                ),
        }
    }

    pub async fn read_by_chapter(&self, pool: &MySqlPool) -> EntityResult<Vec<QuestionEntity>> {
        let question = sqlx
            ::query_as::<_, QuestionEntity>("SELECT * FROM questions WHERE chapter_id = ?")
            .bind(&self.chapter_id)
            .fetch_all(pool).await;
        match question {
            Ok(result) => EntityResult::Success(result),
            Err(e) =>
                EntityResult::Error(
                    DatabaseErrorType::QueryError(
                        "Failed to read question by chapter".to_string(),
                        e.to_string()
                    )
                ),
        }
    }

    pub async fn read_by_course(&self, pool: &MySqlPool) -> EntityResult<Vec<QuestionEntity>> {
        let question = sqlx
            ::query_as::<_, QuestionEntity>("SELECT * FROM questions WHERE course_id = ?")
            .bind(&self.course_id)
            .fetch_all(pool).await;
        match question {
            Ok(result) => EntityResult::Success(result),
            Err(e) =>
                EntityResult::Error(
                    DatabaseErrorType::QueryError(
                        "Failed to read question by course".to_string(),
                        e.to_string()
                    )
                ),
        }
    }

    pub async fn read_by_chapter_course(
        &self,
        pool: &MySqlPool
    ) -> EntityResult<Vec<QuestionEntity>> {
        let question = sqlx
            ::query_as::<_, QuestionEntity>(
                "SELECT * FROM questions WHERE course_id = ? and chapter_id = ?"
            )
            .bind(&self.course_id)
            .bind(&self.chapter_id)
            .fetch_all(pool).await;
        match question {
            Ok(result) => EntityResult::Success(result),
            Err(e) =>
                EntityResult::Error(
                    DatabaseErrorType::QueryError(
                        "Failed to read question by chapter and course".to_string(),
                        e.to_string()
                    )
                ),
        }
    }

    pub async fn read_by_hash(&self, pool: &MySqlPool) -> EntityResult<Option<QuestionEntity>> {
        let question = sqlx
            ::query_as::<_, QuestionEntity>("SELECT * FROM questions WHERE id_hash = ?")
            .bind(&self.id_hash)
            .fetch_optional(pool).await;
        match question {
            Ok(result) => EntityResult::Success(result),
            Err(e) =>
                EntityResult::Error(
                    DatabaseErrorType::QueryError(
                        "Failed to read question by hash".to_string(),
                        e.to_string()
                    )
                ),
        }
    }

    pub async fn read_by_q_hash(&self, pool: &MySqlPool) -> EntityResult<Option<QuestionEntity>> {
        let question = sqlx
            ::query_as::<_, QuestionEntity>("SELECT * FROM questions WHERE que_hash = ?")
            .bind(&self.que_hash)
            .fetch_one(pool).await;
        match question {
            Ok(result) => EntityResult::Success(Some(result)),
            Err(_) => EntityResult::Success(None),
        }
    }

    pub async fn update(&self, pool: &MySqlPool) -> EntityResult<SuccessResultType> {
        let updated_timestamp = OffsetDateTime::now_utc();
        let res = sqlx
            ::query(
                "UPDATE questions SET course_id = ?, chapter_id = ?, id_hash = ?, que_text = ?, que_description = ?, answers = ?, choices = ?, difficulty = ?, diff_reason = ?, ans_explanation = ?, ans_hint = ?, updated_timestamp = ? WHERE question_id = ?"
            )
            .bind(&self.course_id)
            .bind(&self.chapter_id)
            .bind(&self.id_hash)
            .bind(&self.que_text)
            .bind(&self.que_description)
            .bind(&self.answers)
            .bind(&self.choices)
            .bind(&self.difficulty)
            .bind(&self.diff_reason)
            .bind(&self.ans_explanation)
            .bind(&self.ans_hint)
            .bind(updated_timestamp)
            .bind(&self.question_id)
            .execute(pool).await;
        match res {
            Ok(result) =>
                EntityResult::Success(
                    SuccessResultType::Updated(result.last_insert_id(), result.rows_affected())
                ),
            Err(e) =>
                EntityResult::Error(
                    DatabaseErrorType::QueryError(
                        "Failed to update question".to_string(),
                        e.to_string()
                    )
                ),
        }
    }

    pub async fn delete(&self, pool: &MySqlPool) -> EntityResult<SuccessResultType> {
        let res = sqlx
            ::query("DELETE FROM questions WHERE question_id = ?")
            .bind(&self.question_id)
            .execute(pool).await;
        match res {
            Ok(result) =>
                EntityResult::Success(
                    SuccessResultType::Deleted(result.last_insert_id(), result.rows_affected())
                ),
            Err(e) =>
                EntityResult::Error(
                    DatabaseErrorType::QueryError(
                        "Failed to delete question".to_string(),
                        e.to_string()
                    )
                ),
        }
    }
}
