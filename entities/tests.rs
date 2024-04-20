use serde::{Deserialize, Serialize};
use sqlx::MySqlPool;
use validator::Validate;

use crate::result_types::{DatabaseErrorType, EntityResult, SuccessResultType};

#[derive(Validate, Debug, Serialize, Deserialize, PartialEq, Clone, sqlx::FromRow)]
pub struct TestEntity {
    pub test_id: Option<u32>,
    pub id_hash: String,
    pub name: String,
    pub kind: i8,
    pub course_id: u32,
    pub description: String,
}

#[derive(Validate, Debug, Serialize, Deserialize, PartialEq, Clone, sqlx::FromRow)]
pub struct TestQueryModel {
    pub id_hash: String,
    pub test_name: String,
    pub test_kind: i8,
    pub test_description: String,
    pub course_id_hash: Option<String>,
    pub course_name: Option<String>,
    pub chapter_name: Option<String>,
    pub chapter_id_hash: Option<String>,
}

#[derive(Validate, Debug, Serialize, Deserialize, PartialEq, Clone, sqlx::FromRow)]
pub struct TestChapter {
    pub test_id: u32,
    pub chapter_id: u32,
}

#[derive(Validate, Debug, Serialize, Deserialize, PartialEq, Clone, sqlx::FromRow)]
pub struct TestQuestion {
    pub test_id: u32,
    pub question_id: u32,
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
            id_hash: "not-set".to_string(),
            name: "not-set".to_string(),
            kind: 0,
            course_id: 0,
            description: "not-set".to_string(),
        }
    }

    pub async fn create_test(&self, pool: &MySqlPool) -> EntityResult<SuccessResultType> {
        let id_hash = self.id_hash.clone();
        let name = self.name.clone();
        let kind = self.kind;
        let course_id = self.course_id;
        let description = self.description.clone();

        let query = r#"
            INSERT INTO tests (id_hash, name, kind, course_id, description, added_timestamp)
            VALUES (?, ?, ?, ?, ?, ?, ?)
        "#;

        let result = sqlx::query(query).bind(id_hash).bind(name).bind(kind).bind(course_id).bind(description).execute(pool).await;

        match result {
            Ok(r) => EntityResult::Success(SuccessResultType::Created(r.last_insert_id(), r.rows_affected())),
            Err(e) => EntityResult::Error(DatabaseErrorType::QueryError("Failed to create question".to_string(), e.to_string())),
        }
    }

    pub async fn create_test_chapters(&self, pool: &MySqlPool, test_id: u32, chapter_ids: Vec<u32>) -> EntityResult<SuccessResultType> {
        let mut results: Vec<u64> = Vec::new();
        for chapter_id in chapter_ids {
            let result = sqlx::query("INSERT INTO test_chapters (test_id, chapter_id) VALUES (?, ?)")
                .bind(test_id)
                .bind(chapter_id)
                .execute(pool)
                .await;

            match result {
                Ok(r) => results.push(r.last_insert_id()),
                Err(e) => return EntityResult::Error(DatabaseErrorType::QueryError("Failed to create test chapter".to_string(), e.to_string())),
            }
        }

        EntityResult::Success(SuccessResultType::CreatedCollection(results))
    }

    pub async fn create_test_questions(&self, pool: &MySqlPool, test_id: u32, question_ids: Vec<u32>) -> EntityResult<SuccessResultType> {
        let mut results: Vec<u64> = Vec::new();
        for question_id in question_ids {
            let result = sqlx::query("INSERT INTO test_questions (test_id, question_id) VALUES (?, ?)")
                .bind(test_id)
                .bind(question_id)
                .execute(pool)
                .await;

            match result {
                Ok(r) => results.push(r.last_insert_id()),
                Err(e) => return EntityResult::Error(DatabaseErrorType::QueryError("Failed to create test question".to_string(), e.to_string())),
            }
        }

        EntityResult::Success(SuccessResultType::CreatedCollection(results))
    }

    // get all tests include course and chapter names and ids
    pub async fn find_all(&self, pool: &MySqlPool) -> EntityResult<Vec<TestQueryModel>> {
        let tests = sqlx::query_as::<_, TestQueryModel>(
            r#"
            SELECT t.test_id, t.name as test_name, t.kind as test_kind, t.description as test_description, t.course_id, c.name as course_name, ch.chapter_id, ch.name as chapter_name
            FROM tests t
            JOIN test_chapters tc ON t.test_id = tc.test_id
            JOIN chapters ch ON tc.chapter_id = ch.chapter_id
            JOIN courses c ON t.course_id = c.course_id
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
        let test = sqlx::query_as::<_, TestQueryModel>("SELECT * FROM tests WHERE id_hash = ?")
            .bind(&self.id_hash)
            .fetch_optional(pool)
            .await;
        match test {
            Ok(result) => EntityResult::Success(result),
            Err(e) => EntityResult::Error(DatabaseErrorType::QueryError("Failed to read test by hash".to_string(), e.to_string())),
        }
    }

    // get all tests in a course
    // capture test_id, id_hash, name, kind, course_id, course_name, added_timestamp, description in a new struct
    // then return the struct as a vector
    pub async fn find_by_course(&self, pool: &MySqlPool, course_id_hash: String) -> EntityResult<Vec<TestQueryModel>> {
        let tests = sqlx::query_as::<_, TestQueryModel>(
            r#"
            SELECT t.test_id, t.id_hash, t.name, t.kind, t.course_id, c.name as course_name, t.added_timestamp, t.description
            FROM tests t
            JOIN courses c ON t.course_id = c.course_id
            WHERE t.course_id = ?
        "#,
        )
        .bind(course_id_hash)
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
    pub async fn find_by_chapter(&self, pool: &MySqlPool, chapter_id_hash: String) -> EntityResult<Vec<TestQueryModel>> {
        let tests = sqlx::query_as::<_, TestQueryModel>(
            r#"
            SELECT t.test_id, t.name as test_name, t.kind as test_kind, t.description as test_description, t.course_id, c.name as course_name, ch.chapter_id, ch.name as chapter_name
            FROM tests t
            JOIN test_chapters tc ON t.test_id = tc.test_id
            JOIN chapters ch ON tc.chapter_id = ch.chapter_id
            JOIN courses c ON t.course_id = c.course_id
            WHERE ch.chapter_id = ?
        "#,
        )
        .bind(&chapter_id_hash)
        .fetch_all(pool)
        .await;

        match tests {
            Ok(result) => EntityResult::Success(result),
            Err(e) => EntityResult::Error(DatabaseErrorType::QueryError(format!("Failed to read tests chapter: {chapter_id_hash}"), e.to_string())),
        }
    }
}
