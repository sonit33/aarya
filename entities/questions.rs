use aarya_utils::{hash_ops, random::randomize_u32s};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use sqlx::MySqlPool;
use validator::{Validate, ValidateLength};

use crate::result_types::{DatabaseErrorType, EntityResult, SuccessResultType};

#[derive(Debug, Serialize, Deserialize)]
pub struct Choice {
    id: String,
    text: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Answer {
    id: String,
}

#[derive(Validate, Debug, Serialize, Deserialize, PartialEq, Clone, sqlx::FromRow)]
pub struct QuestionEntity {
    pub question_id: Option<u32>,
    pub course_id: u32,
    pub chapter_id: Option<u32>,
    pub topic_id: Option<u32>,
    pub que_text: String,
    pub que_description: String,
    pub choices: Value, // Assuming JSON structure is [{ "id": "", "text": "" }]
    pub answers: Value, // Assuming JSON structure is [{ "id": "" }]
    pub ans_explanation: String,
    pub ans_hint: String,
    pub difficulty: i8,
    pub diff_reason: String,
    pub que_hash: Option<String>,
}

#[derive(Validate, Debug, Serialize, Deserialize, PartialEq, Clone, sqlx::FromRow)]
pub struct QuestionQueryModel {
    pub question_id: u32,
    pub course_id: u32,
    pub chapter_id: Option<u32>,
    pub topic_id: Option<u32>,
    pub que_text: String,
    pub que_description: String,
    pub choices: String,
    pub radio: bool,
    pub que_difficulty: u8,
    pub diff_reason: String,
    pub ans_explanation: String,
    pub ans_hint: String,
    pub course_name: Option<String>,
    pub chapter_name: Option<String>,
}

#[derive(Validate, Debug, Serialize, Deserialize, PartialEq, Clone, sqlx::FromRow)]
pub struct QuestionIdQueryModel {
    pub question_id: u32,
}

impl QuestionEntity {
    pub fn new() -> Self {
        QuestionEntity {
            question_id: Some(0),
            course_id: 0,
            chapter_id: Some(0),
            topic_id: Some(0),
            que_text: "not-set".to_string(),
            que_description: "not-set".to_string(),
            choices: json!([{"id":"", "text":"not-set"}]),
            answers: json!([{"id":""}]),
            ans_explanation: "not-set".to_string(),
            ans_hint: "not-set".to_string(),
            difficulty: 0,
            diff_reason: "not-set".to_string(),
            que_hash: Some(String::from("random")),
        }
    }
}

impl Default for QuestionEntity {
    fn default() -> Self {
        Self::new()
    }
}

impl QuestionEntity {
    pub async fn create(
        &self,
        pool: &MySqlPool,
    ) -> EntityResult<SuccessResultType> {
        let que_hash = hash_ops::string_hasher(self.que_text.to_lowercase().as_str());
        let radio = self.answers.as_array().length().unwrap() == 1; // determines showing radio buttons or checkboxes
        let res = sqlx::query(
            "INSERT INTO questions (
                    question_id,
                    course_id, 
                    chapter_id,
                    topic_id, 
                    que_text, 
                    que_description, 
                    answers, 
                    radio,
                    choices, 
                    difficulty, 
                    diff_reason, 
                    ans_explanation, 
                    ans_hint, 
                    que_hash) 
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
        )
        .bind(self.question_id)
        .bind(self.course_id)
        .bind(self.chapter_id)
        .bind(self.topic_id)
        .bind(&self.que_text)
        .bind(&self.que_description)
        .bind(&self.answers)
        .bind(radio)
        .bind(&self.choices)
        .bind(self.difficulty)
        .bind(&self.diff_reason)
        .bind(&self.ans_explanation)
        .bind(&self.ans_hint)
        .bind(que_hash)
        .execute(pool)
        .await;
        match res {
            Ok(result) => EntityResult::Success(SuccessResultType::Created(result.last_insert_id(), result.rows_affected())),
            Err(e) => EntityResult::Error(DatabaseErrorType::QueryError("Failed to create question".to_string(), e.to_string())),
        }
    }

    // read all questions, join with course table to get course and question details. // Do not use * in query, instead use column names
    pub async fn find_by_course(
        &self,
        pool: &MySqlPool,
        course_id: u32,
    ) -> EntityResult<Vec<QuestionQueryModel>> {
        let questions = sqlx::query_as::<_, QuestionQueryModel>(
            r#"
                SELECT 
                    q.question_id, 
                    q.course_id, 
                    q.chapter_id, 
                    q.topic_id,
                    q.que_text, 
                    q.que_description, 
                    q.choices, 
                    q.radio,
                    q.difficulty, 
                    q.diff_reason, 
                    q.ans_explanation, 
                    q.ans_hint, 
                    c.course_name
                FROM questions q 
                JOIN course c 
                    ON q.course_id = c.course_id
                where c.id_hash = ?
            "#,
        )
        .bind(course_id)
        .fetch_all(pool)
        .await;
        match questions {
            Ok(result) => EntityResult::Success(result),
            Err(e) => EntityResult::Error(DatabaseErrorType::QueryError("Failed to read questions with course".to_string(), e.to_string())),
        }
    }

    // Read all questions, join with course and chapter tables to get course, chapter, and question details
    // Do not use * in query, instead use column names
    pub async fn find_by_chapter(
        &self,
        pool: &MySqlPool,
        chapter_id: u32,
    ) -> EntityResult<Vec<QuestionQueryModel>> {
        let questions = sqlx::query_as::<_, QuestionQueryModel>(
            r#"
                SELECT 
                    q.question_id, 
                    q.course_id, 
                    q.chapter_id,
                    q.topic_id, 
                    q.que_text, 
                    q.que_description, 
                    q.choices, 
                    q.radio,
                    q.difficulty, 
                    q.diff_reason,
                    q.ans_explanation, 
                    q.ans_hint, 
                    c.course_name, 
                    ch.chapter_name
                FROM questions q
                JOIN courses c
                    ON q.course_id = c.course_id
                JOIN chapters ch
                    ON q.chapter_id = ch.chapter_id
                where ch.chapter_id = ?
            "#,
        )
        .bind(chapter_id)
        .fetch_all(pool)
        .await;
        match questions {
            Ok(result) => EntityResult::Success(result),
            Err(e) => EntityResult::Error(DatabaseErrorType::QueryError("Failed to read questions with course and chapter".to_string(), e.to_string())),
        }
    }

    pub async fn find_one(
        &self,
        pool: &MySqlPool,
    ) -> EntityResult<Option<QuestionQueryModel>> {
        let question = sqlx::query_as::<_, QuestionQueryModel>(
            r#"
            SELECT 
                q.question_id, 
                q.course_id, 
                q.chapter_id, 
                q.topic_id,
                q.que_text, 
                q.que_description, 
                q.choices, 
                q.radio,
                q.difficulty, 
                q.diff_reason,
                q.ans_explanation, 
                q.ans_hint, 
                c.course_name, 
                ch.chapter_name
            FROM questions q 
            WHERE question_id = ?"#,
        )
        .bind(self.question_id)
        .fetch_optional(pool)
        .await;
        match question {
            Ok(result) => EntityResult::Success(result),
            Err(e) => EntityResult::Error(DatabaseErrorType::QueryError("Failed to read question by hash".to_string(), e.to_string())),
        }
    }

    pub async fn find_duplicate(
        &self,
        pool: &MySqlPool,
    ) -> EntityResult<Option<QuestionQueryModel>> {
        let question = sqlx::query_as::<_, QuestionQueryModel>(r#"SELECT q.question_id FROM questions q WHERE que_hash = ?"#)
            .bind(&self.que_hash)
            .fetch_one(pool)
            .await;
        match question {
            Ok(result) => EntityResult::Success(Some(result)),
            Err(_) => EntityResult::Success(None),
        }
    }

    pub async fn find_top_n(
        &self,
        pool: &MySqlPool,
        limit: u32,
    ) -> EntityResult<Vec<QuestionQueryModel>> {
        let questions = sqlx::query_as::<_, QuestionQueryModel>(
            r#"
                SELECT 
                    q.question_id, 
                    q.course_id, 
                    q.chapter_id,
                    q.topic_id, 
                    q.que_text, 
                    q.que_description, 
                    q.choices, 
                    q.radio,
                    q.difficulty, 
                    q.diff_reason,
                    q.ans_explanation, 
                    q.ans_hint, 
                    t.topic_name,
                    c.course_name, 
                    ch.chapter_name
                FROM questions q
                JOIN courses c
                    ON q.course_id = c.course_id
                JOIN chapters ch
                    ON q.chapter_id = ch.chapter_id
                JOIN topics t
                    ON t.topic_id = q.topic_id
                WHERE q.difficulty = ? 
                    and q.course_id = ? 
                    and q.chapter_id = ? 
                    and q.topic_id = ?
                LIMIT ?
            "#,
        )
        .bind(self.difficulty)
        .bind(self.course_id)
        .bind(self.chapter_id)
        .bind(self.topic_id)
        .bind(limit)
        .fetch_all(pool)
        .await;
        match questions {
            Ok(result) => EntityResult::Success(result),
            Err(e) => EntityResult::Error(DatabaseErrorType::QueryError("Failed to read questions with course and chapter".to_string(), e.to_string())),
        }
    }

    pub async fn find_random_questions(
        &self,
        pool: &MySqlPool,
        limit: u32,
    ) -> EntityResult<Vec<u32>> {
        let questions = sqlx::query_as::<_, QuestionIdQueryModel>(
            r#"
                SELECT 
                    q.question_id 
                FROM questions q
                JOIN courses c
                    ON q.course_id = c.course_id
                JOIN chapters ch
                    ON q.chapter_id = ch.chapter_id
                JOIN topics t
                    ON t.topic_id = q.topic_id
                WHERE q.difficulty = ? 
                    and q.course_id = ? 
                    and q.chapter_id = ? 
                    and q.topic_id = ?
            "#,
        )
        .bind(self.difficulty)
        .bind(self.course_id)
        .bind(self.chapter_id)
        .bind(self.topic_id)
        .fetch_all(pool)
        .await;

        match questions {
            Ok(result) => {
                // EntityResult::Success(result)
                let mut question_ids: Vec<u32> = Vec::new();
                result.clone().into_iter().for_each(|question| {
                    question_ids.push(question.question_id);
                });
                EntityResult::Success(randomize_u32s(question_ids, limit))
            }
            Err(e) => EntityResult::Error(DatabaseErrorType::QueryError("Failed to read questions with course and chapter".to_string(), e.to_string())),
        }
    }

    pub async fn update(
        &self,
        pool: &MySqlPool,
    ) -> EntityResult<SuccessResultType> {
        let res = sqlx::query(
            r#"
                UPDATE questions SET 
                    course_id = ?, 
                    chapter_id = ?, 
                    que_text = ?, 
                    que_description = ?, 
                    answers = ?, 
                    choices = ?, 
                    difficulty = ?, 
                    diff_reason = ?, 
                    ans_explanation = ?, 
                    ans_hint = ?
                WHERE question_id = ?"#,
        )
        .bind(self.course_id)
        .bind(self.chapter_id)
        .bind(&self.que_text)
        .bind(&self.que_description)
        .bind(&self.answers)
        .bind(&self.choices)
        .bind(self.difficulty)
        .bind(&self.diff_reason)
        .bind(&self.ans_explanation)
        .bind(&self.ans_hint)
        .bind(self.question_id)
        .execute(pool)
        .await;
        match res {
            Ok(result) => EntityResult::Success(SuccessResultType::Updated(result.last_insert_id(), result.rows_affected())),
            Err(e) => EntityResult::Error(DatabaseErrorType::QueryError("Failed to update question".to_string(), e.to_string())),
        }
    }

    pub async fn delete(
        &self,
        pool: &MySqlPool,
    ) -> EntityResult<SuccessResultType> {
        let res = sqlx::query("DELETE FROM questions WHERE question_id = ?").bind(self.question_id).execute(pool).await;
        match res {
            Ok(result) => EntityResult::Success(SuccessResultType::Deleted(result.last_insert_id(), result.rows_affected())),
            Err(e) => EntityResult::Error(DatabaseErrorType::QueryError("Failed to delete question".to_string(), e.to_string())),
        }
    }
}
