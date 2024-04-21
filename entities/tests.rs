use serde::{Deserialize, Serialize};
use sqlx::MySqlPool;

use crate::result_types::{DatabaseErrorType, EntityResult, SuccessResultType};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, sqlx::FromRow)]
pub struct TestEntity {
    pub test_id: Option<u32>,
    pub name: String,
    pub test_kind: String,
    pub course_id: u32,
    pub chapter_id: Option<u32>,
    pub topic_id: Option<u32>,
    pub description: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, sqlx::FromRow)]
pub struct TestQueryModel {
    pub test_name: String,
    pub test_kind: String,
    pub test_description: String,
    pub course_id: Option<u32>,
    pub course_name: Option<String>,
    pub chapter_id: Option<u32>,
    pub chapter_name: Option<String>,
    pub topic_id: Option<u32>,
    pub topic_name: Option<String>,
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
            name: "not-set".to_string(),
            test_kind: String::from("quiz"),
            course_id: 0,
            chapter_id: Some(0),
            topic_id: Some(0),
            description: "not-set".to_string(),
        }
    }

    pub async fn create_test(&self, pool: &MySqlPool) -> EntityResult<SuccessResultType> {
        let query = r#"
            INSERT INTO tests (name, test_kind, course_id, chapter_id, topic_id, description)
            VALUES (?, ?, ?, ?, ?, ?)
        "#;

        let result = sqlx::query(query)
            .bind(&self.name)
            .bind(&self.test_kind)
            .bind(self.course_id)
            .bind(self.chapter_id)
            .bind(self.topic_id)
            .bind(&self.description)
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
                t.test_id, t.name as test_name, t.kind as test_kind, t.description as test_description, t.course_id, c.name as course_name, ch.chapter_id, ch.name as chapter_name, tp.topic_id, tp.name as topic_name
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
                t.test_id, t.name as test_name, t.kind as test_kind, t.description as test_description, t.course_id, c.name as course_name, ch.chapter_id, ch.name as chapter_name, tp.topic_id, tp.name as topic_name
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
                t.test_id, t.name as test_name, t.kind as test_kind, t.description as test_description, t.course_id, c.name as course_name, ch.chapter_id, ch.name as chapter_name, tp.topic_id, tp.name as topic_name
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
                t.test_id, t.name as test_name, t.kind as test_kind, t.description as test_description, t.course_id, c.name as course_name, ch.chapter_id, ch.name as chapter_name, tp.topic_id, tp.name as topic_name
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
