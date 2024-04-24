use serde::{Deserialize, Serialize};
use sqlx::MySqlPool;

use crate::result_types::{DatabaseErrorType, EntityResult, SuccessResultType};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, sqlx::FromRow)]
pub struct TestEntity {
    pub test_id: Option<u32>,
    pub student_id: u32,
    pub course_id: u32,
    pub chapter_id: Option<u32>,
    pub topic_id: Option<u32>,
    pub test_difficulty: u8,
    pub test_length: u8,
    pub test_state: u8,
}

/// association table for tests and questions
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, sqlx::FromRow)]
pub struct TestQuestionsEntity {
    pub test_id: u32,
    pub question_id: u32,
    pub state: u8,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct TestMutationModel {
    pub course_id: u32,
    pub chapter_id: Option<u32>,
    pub topic_id: Option<u32>,
    pub test_difficulty: u8,
    pub test_length: u8,
    pub test_state: u8,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, sqlx::FromRow)]
pub struct TestQueryModel {
    pub test_name: String,
    pub test_description: String,
    pub course_id: Option<u32>,
    pub course_name: Option<String>,
    pub chapter_id: Option<u32>,
    pub chapter_name: Option<String>,
    pub topic_id: Option<u32>,
    pub topic_name: Option<String>,
    pub test_difficulty: u8,
    pub test_length: String,
    pub test_state: u8,
}

impl Default for TestEntity {
    fn default() -> Self {
        Self::new()
    }
}

impl TestEntity {
    pub fn new() -> Self {
        TestEntity {
            test_id: Some(0),
            course_id: 0,
            student_id: 0,
            chapter_id: Some(0),
            topic_id: Some(0),
            test_difficulty: 0,
            test_length: 0,
            test_state: 0,
        }
    }

    pub async fn create_test(&self, pool: &MySqlPool) -> EntityResult<SuccessResultType> {
        let query = r#"
            INSERT INTO tests (course_id, student_id, chapter_id, topic_id, test_difficulty, test_length, test_state)
            VALUES (?, ?, ?, ?, ?, ?, ?)
        "#;

        let result = sqlx::query(query)
            .bind(self.course_id)
            .bind(self.student_id)
            .bind(self.chapter_id)
            .bind(self.topic_id)
            .bind(self.test_difficulty)
            .bind(self.test_length)
            .bind(self.test_state)
            .execute(pool)
            .await;

        match result {
            Ok(r) => EntityResult::Success(SuccessResultType::Created(r.last_insert_id(), r.rows_affected())),
            Err(e) => EntityResult::Error(DatabaseErrorType::QueryError("Failed to create question".to_string(), e.to_string())),
        }
    }

    // get all tests include course and chapter names and ids
    pub async fn find_all(&self, pool: &MySqlPool) -> EntityResult<Vec<TestQueryModel>> {
        let tests = sqlx::query_as::<_, TestQueryModel>(
            r#"
            SELECT 
                test_id, test_name, test_length, test_difficulty, test_state, test_description, c.course_id, c.course_name, ch.chapter_id, ch.chapter_name, tp.topic_id, tp.topic_name
            FROM tests t
            JOIN courses c ON t.course_id = c.course_id
            JOIN chapters ch on t.chapter_id = ch.chapter_id
            JOIN topics tp on t.topic_id = tp.topic_id
        "#,
        )
        .fetch_all(pool)
        .await;

        match tests {
            Ok(result) => EntityResult::Success(result),
            Err(e) => EntityResult::Error(DatabaseErrorType::QueryError("Failed to read all tests".to_string(), e.to_string())),
        }
    }

    // get a test by id_hash
    pub async fn find_one(&self, pool: &MySqlPool) -> EntityResult<Option<TestQueryModel>> {
        let tests = sqlx::query_as::<_, TestQueryModel>(
            r#"
            SELECT 
                test_id, test_name, test_length, test_difficulty, test_state, test_description, c.course_id, c.course_name, ch.chapter_id, ch.chapter_name, tp.topic_id, tp.topic_name
            FROM tests t
            JOIN courses c ON t.course_id = c.course_id
            JOIN chapters ch on t.chapter_id = ch.chapter_id
            JOIN topics tp on t.topic_id = tp.topic_id
            where t.test_id = ?
        "#,
        )
        .bind(self.test_id)
        .fetch_optional(pool)
        .await;

        match tests {
            Ok(result) => EntityResult::Success(result),
            Err(e) => EntityResult::Error(DatabaseErrorType::QueryError("Failed to read all tests".to_string(), e.to_string())),
        }
    }

    // get all tests in a course
    // capture test_id, id_hash, name, kind, course_id, course_name, added_timestamp, description in a new struct
    // then return the struct as a vector
    pub async fn find_by_course(&self, pool: &MySqlPool) -> EntityResult<Vec<TestQueryModel>> {
        let tests = sqlx::query_as::<_, TestQueryModel>(
            r#"
            SELECT 
                test_id, test_name, test_length, test_difficulty, test_state, test_description, c.course_id, c.course_name, ch.chapter_id, ch.chapter_name, tp.topic_id, tp.topic_name
            FROM tests t
            JOIN courses c ON t.course_id = c.course_id
            JOIN chapters ch on t.chapter_id = ch.chapter_id
            JOIN topics tp on t.topic_id = tp.topic_id
            WHERE t.course_id = ?
        "#,
        )
        .bind(self.course_id)
        .fetch_all(pool)
        .await;

        match tests {
            Ok(result) => EntityResult::Success(result),
            Err(e) => EntityResult::Error(DatabaseErrorType::QueryError("Failed to read test by course".to_string(), e.to_string())),
        }
    }

    // get all tests in a chapter in a course by joining test, test_chapters, chapter, and course table to return course and chapter names
    // captures test_id, test_name, test_kind, test_description, course_id, course_name, chapter_id, chapter_name in a new struct
    // then return the struct as a vector
    pub async fn find_by_chapter(&self, pool: &MySqlPool, chapter_id: u32) -> EntityResult<Vec<TestQueryModel>> {
        let tests = sqlx::query_as::<_, TestQueryModel>(
            r#"
            SELECT 
                test_id, test_name, test_length, test_difficulty, test_state, test_description, c.course_id, c.course_name, ch.chapter_id, ch.chapter_name, tp.topic_id, tp.topic_name
            FROM tests t
            JOIN courses c ON t.course_id = c.course_id
            JOIN chapters ch on t.chapter_id = ch.chapter_id
            JOIN topics tp on t.topic_id = tp.topic_id
            WHERE ch.chapter_id = ?
        "#,
        )
        .bind(chapter_id)
        .fetch_all(pool)
        .await;

        match tests {
            Ok(result) => EntityResult::Success(result),
            Err(e) => EntityResult::Error(DatabaseErrorType::QueryError(format!("Failed to read tests chapter: {chapter_id}"), e.to_string())),
        }
    }
}

impl TestQuestionsEntity {
    pub fn new() -> Self {
        TestQuestionsEntity { test_id: 0, question_id: 0, state: 0 }
    }

    pub async fn create(&self, pool: &MySqlPool) -> EntityResult<SuccessResultType> {
        let query = r#"
            INSERT INTO test_questions (test_id, question_id, state)
            VALUES (?, ?, ?)
        "#;

        let result = sqlx::query(query).bind(self.test_id).bind(self.question_id).bind(self.state).execute(pool).await;

        match result {
            Ok(r) => EntityResult::Success(SuccessResultType::Created(r.last_insert_id(), r.rows_affected())),
            Err(e) => EntityResult::Error(DatabaseErrorType::QueryError("Failed to create test question".to_string(), e.to_string())),
        }
    }

    pub async fn find_top(&self, pool: &MySqlPool) -> EntityResult<Vec<TestQuestionsEntity>> {
        let questions = sqlx::query_as::<_, TestQuestionsEntity>(
            r#"
            SELECT test_id, question_id, state
            FROM test_questions
            WHERE test_id = ? and state = 0
            LIMIT 1
        "#,
        )
        .bind(self.test_id)
        .fetch_all(pool)
        .await;

        match questions {
            Ok(result) => EntityResult::Success(result),
            Err(e) => EntityResult::Error(DatabaseErrorType::QueryError("Failed to read test questions".to_string(), e.to_string())),
        }
    }
}

impl Default for TestQuestionsEntity {
    fn default() -> Self {
        Self::new()
    }
}
