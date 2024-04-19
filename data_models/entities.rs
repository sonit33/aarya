use aarya_utils::hash_ops;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use sqlx::MySqlPool;
use time::OffsetDateTime;

#[derive(Debug, sqlx::FromRow)]
pub struct ChapterEntity {
    pub chapter_id: Option<u32>,
    pub id_hash: String,
    pub course_id: Option<u32>,
    pub name: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, sqlx::FromRow)]
pub struct ChapterWithCourse {
    pub chapter_id: u32,
    pub id_hash: String,
    pub name: String,
    pub description: String,
    pub course_id: u32,
    pub course_name: Option<String>,
    pub course_id_hash: Option<String>,
}

impl ChapterEntity {
    pub fn new() -> Self {
        ChapterEntity {
            chapter_id: Some(0),
            id_hash: "not-set".to_string(),
            course_id: None,
            name: None,
            description: None,
        }
    }

    pub async fn create_chapter(&self, pool: &MySqlPool) -> EntityResult<SuccessResultType> {
        let id_hash = self.id_hash.clone();
        let course_id = self.course_id;
        let name = self.name.clone();
        let description = self.description.clone();

        let query = r#"
            INSERT INTO chapter (id_hash, course_id, name, description)
            VALUES (?, ?, ?, ?)
        "#;

        match sqlx::query(query).bind(id_hash).bind(course_id).bind(name).bind(description).execute(pool).await {
            Ok(r) => EntityResult::Success(SuccessResultType::Created(r.last_insert_id(), r.rows_affected())),
            Err(e) => EntityResult::Error(DatabaseErrorType::QueryError("Error creating chapter".to_string(), e.to_string())),
        }
    }

    // get all chapters by joining with the course table to get course and chapter details incluidng course name
    pub async fn get_chapters_by_course(&self, pool: &MySqlPool, id_hash: String) -> EntityResult<Vec<ChapterWithCourse>> {
        let query = r#"
            SELECT c.chapter_id, c.id_hash, c.course_id, co.name as course_name, co.id_hash as course_id_hash, c.name, c.description
            FROM chapters c
                    JOIN courses co ON c.course_id = co.course_id
            where co.id_hash = ?
        "#;

        match sqlx::query_as::<_, ChapterWithCourse>(query).bind(&id_hash).fetch_all(pool).await {
            Ok(chapters) => EntityResult::Success(chapters),
            Err(e) => EntityResult::Error(DatabaseErrorType::QueryError("Error fetching chapters".to_string(), e.to_string())),
        }
    }
}

impl Default for ChapterEntity {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, sqlx::FromRow)]
pub struct CourseEntity {
    pub course_id: Option<u32>,
    pub name: String,
    pub id_hash: String,
    pub added_timestamp: Option<OffsetDateTime>,
    pub updated_timestamp: Option<OffsetDateTime>,
    pub description: String,
}

impl CourseEntity {
    pub fn new() -> Self {
        CourseEntity {
            course_id: Some(0),
            name: "not-set".to_string(),
            id_hash: "not-set".to_string(),
            added_timestamp: None,
            updated_timestamp: None,
            description: "not-set".to_string(),
        }
    }

    // create a new course
    pub async fn create_course(&self, pool: &MySqlPool) -> EntityResult<SuccessResultType> {
        let name = self.name.clone();
        let id_hash = self.id_hash.clone();
        let description = self.description.clone();

        let query = r#"
            INSERT INTO courses (name, id_hash, description)
            VALUES (?, ?, ?)
        "#;

        match sqlx::query(query).bind(name).bind(id_hash).bind(description).execute(pool).await {
            Ok(_) => EntityResult::Success(SuccessResultType::Created(0, 0)),
            Err(e) => EntityResult::Error(DatabaseErrorType::QueryError("Error creating course".to_string(), e.to_string())),
        }
    }

    // get all courses
    pub async fn read_all(&self, pool: &MySqlPool) -> EntityResult<Vec<CourseEntity>> {
        let query = r#"
            SELECT * FROM courses
        "#;

        match sqlx::query_as::<_, CourseEntity>(query).fetch_all(pool).await {
            Ok(courses) => EntityResult::Success(courses),
            Err(e) => EntityResult::Error(DatabaseErrorType::QueryError("Error fetching courses".to_string(), e.to_string())),
        }
    }
}

impl Default for CourseEntity {
    fn default() -> Self {
        Self::new()
    }
}

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
    pub question_id: Option<u32>,
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

#[derive(Debug, sqlx::FromRow)]
pub struct QuestionWithCourseChapter {
    pub question_id: u32,
    pub course_id: u32,
    pub chapter_id: u32,
    pub id_hash: String,
    pub que_text: String,
    pub que_description: String,
    pub choices: String,
    pub difficulty: i8,
    pub diff_reason: String,
    pub ans_explanation: String,
    pub ans_hint: String,
    pub course_name: Option<String>,
    pub chapter_name: Option<String>,
}

impl QuestionEntity {
    pub fn new() -> Self {
        QuestionEntity {
            question_id: Some(0),
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

impl Default for QuestionEntity {
    fn default() -> Self {
        Self::new()
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
            .bind(self.course_id)
            .bind(self.chapter_id)
            .bind(&self.id_hash)
            .bind(&self.que_text)
            .bind(&self.que_description)
            .bind(&self.answers)
            .bind(&self.choices)
            .bind(self.difficulty)
            .bind(&self.diff_reason)
            .bind(&self.ans_explanation)
            .bind(&self.ans_hint)
            .bind(que_hash)
            .bind(added_timestamp)
            .execute(pool).await;
        match res {
            Ok(result) => EntityResult::Success(SuccessResultType::Created(result.last_insert_id(), result.rows_affected())),
            Err(e) => EntityResult::Error(DatabaseErrorType::QueryError("Failed to create question".to_string(), e.to_string())),
        }
    }

    pub async fn read_all(&self, pool: &MySqlPool) -> EntityResult<Vec<QuestionWithCourseChapter>> {
        let questions = sqlx::query_as::<_, QuestionWithCourseChapter>(
            r#"SELECT q.question_id, q.course_id, q.chapter_id, q.id_hash, q.que_text, q.que_description, q.choices, 
            q.difficulty, q.diff_reason, q.ans_explanation, q.ans_hint 
            FROM questions q"#,
        )
        .fetch_all(pool)
        .await;
        match questions {
            Ok(result) => EntityResult::Success(result),
            Err(e) => EntityResult::Error(DatabaseErrorType::QueryError("Failed to read questions".to_string(), e.to_string())),
        }
    }

    // read all questions, join with course table to get course and question details. // Do not use * in query, instead use column names
    pub async fn read_all_with_course(&self, pool: &MySqlPool) -> EntityResult<Vec<QuestionWithCourseChapter>> {
        let questions = sqlx::query_as::<_, QuestionWithCourseChapter>(
            r#"
        SELECT q.question_id, q.course_id, q.chapter_id, q.id_hash, q.que_text, q.que_description, q.choices, 
        q.difficulty, q.diff_reason, q.ans_explanation, q.ans_hint, c.name as course_name 
        FROM questions q 
        JOIN course c 
            ON q.course_id = c.course_id"#,
        )
        .fetch_all(pool)
        .await;
        match questions {
            Ok(result) => EntityResult::Success(result),
            Err(e) => EntityResult::Error(DatabaseErrorType::QueryError("Failed to read questions with course".to_string(), e.to_string())),
        }
    }

    // Read all questions, join with course and chapter tables to get course, chapter, and question details
    // Do not use * in query, instead use column names
    pub async fn read_all_with_course_chapter(&self, pool: &MySqlPool) -> EntityResult<Vec<QuestionWithCourseChapter>> {
        let questions = sqlx::query_as::<_, QuestionWithCourseChapter>(
            r#"
        SELECT q.question_id, q.course_id, q.chapter_id, q.id_hash, q.que_text, q.que_description, q.choices, q.difficulty, q.diff_reason,
        q.ans_explanation, q.ans_hint, c.name as course_name, ch.name as chapter_name
        FROM questions q
        JOIN courses c
            ON q.course_id = c.course_id
        JOIN chapters ch
            ON q.chapter_id = ch.chapter_id
            "#,
        )
        .fetch_all(pool)
        .await;
        match questions {
            Ok(result) => EntityResult::Success(result),
            Err(e) => EntityResult::Error(DatabaseErrorType::QueryError("Failed to read questions with course and chapter".to_string(), e.to_string())),
        }
    }

    pub async fn read_by_hash(&self, pool: &MySqlPool) -> EntityResult<Option<QuestionWithCourseChapter>> {
        let question = sqlx::query_as::<_, QuestionWithCourseChapter>(
            r#"SELECT q.question_id, q.course_id, q.chapter_id, q.id_hash, q.que_text, q.que_description, q.choices, q.difficulty, q.diff_reason,
            q.ans_explanation, q.ans_hint, c.name as course_name, ch.name as chapter_name
            FROM questions q WHERE id_hash = ?"#,
        )
        .bind(&self.id_hash)
        .fetch_optional(pool)
        .await;
        match question {
            Ok(result) => EntityResult::Success(result),
            Err(e) => EntityResult::Error(DatabaseErrorType::QueryError("Failed to read question by hash".to_string(), e.to_string())),
        }
    }

    pub async fn read_by_q_hash(&self, pool: &MySqlPool) -> EntityResult<Option<QuestionWithCourseChapter>> {
        let question = sqlx::query_as::<_, QuestionWithCourseChapter>(
            r#"SELECT q.question_id, q.course_id, q.chapter_id, q.id_hash, q.que_text, q.que_description, q.choices, q.difficulty, q.diff_reason,
            q.ans_explanation, q.ans_hint, c.name as course_name, ch.name as chapter_name
            FROM questions q WHERE que_hash = ?"#,
        )
        .bind(&self.que_hash)
        .fetch_one(pool)
        .await;
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
            .bind(self.course_id)
            .bind(self.chapter_id)
            .bind(&self.id_hash)
            .bind(&self.que_text)
            .bind(&self.que_description)
            .bind(&self.answers)
            .bind(&self.choices)
            .bind(self.difficulty)
            .bind(&self.diff_reason)
            .bind(&self.ans_explanation)
            .bind(&self.ans_hint)
            .bind(updated_timestamp)
            .bind(self.question_id)
            .execute(pool).await;
        match res {
            Ok(result) => EntityResult::Success(SuccessResultType::Updated(result.last_insert_id(), result.rows_affected())),
            Err(e) => EntityResult::Error(DatabaseErrorType::QueryError("Failed to update question".to_string(), e.to_string())),
        }
    }

    pub async fn delete(&self, pool: &MySqlPool) -> EntityResult<SuccessResultType> {
        let res = sqlx::query("DELETE FROM questions WHERE question_id = ?").bind(self.question_id).execute(pool).await;
        match res {
            Ok(result) => EntityResult::Success(SuccessResultType::Deleted(result.last_insert_id(), result.rows_affected())),
            Err(e) => EntityResult::Error(DatabaseErrorType::QueryError("Failed to delete question".to_string(), e.to_string())),
        }
    }
}

#[derive(Debug)]
pub enum DatabaseErrorType {
    NotFound(String, String),
    ConnectionError(String, String),
    QueryError(String, String),
}

#[derive(Debug)]
pub enum SuccessResultType {
    Created(u64, u64),
    CreatedCollection(Vec<u64>),
    Updated(u64, u64),
    Deleted(u64, u64),
}

#[derive(Debug)]
pub enum EntityResult<T> {
    Success(T),
    Error(DatabaseErrorType),
}

#[derive(Debug, sqlx::FromRow)]
pub struct TestEntity {
    pub test_id: Option<u32>,
    pub id_hash: String,
    pub name: String,
    pub kind: i8,
    pub course_id: u32,
    pub added_timestamp: Option<OffsetDateTime>,
    pub description: String,
}

#[derive(Debug, sqlx::FromRow)]
pub struct TestWithCourseChapter {
    pub test_id: u32,
    pub test_name: String,
    pub test_kind: i8,
    pub test_description: String,
    pub course_id: u32,
    pub course_name: Option<String>,
    pub chapter_id: Option<u32>,
    pub chapter_name: Option<String>,
}

#[derive(Debug, sqlx::FromRow)]
pub struct TestChapter {
    pub test_id: u32,
    pub chapter_id: u32,
}

#[derive(Debug, sqlx::FromRow)]
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
            added_timestamp: None,
            description: "not-set".to_string(),
        }
    }

    pub async fn create_test(&self, pool: &MySqlPool) -> EntityResult<SuccessResultType> {
        let id_hash = self.id_hash.clone();
        let name = self.name.clone();
        let kind = self.kind;
        let course_id = self.course_id;
        let description = self.description.clone();
        let added_timestamp = self.added_timestamp;

        let query = r#"
            INSERT INTO test (id_hash, name, kind, course_id, description, added_timestamp)
            VALUES (?, ?, ?, ?, ?, ?, ?)
        "#;

        let result = sqlx::query(query)
            .bind(id_hash)
            .bind(name)
            .bind(kind)
            .bind(course_id)
            .bind(description)
            .bind(added_timestamp)
            .execute(pool)
            .await;

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
    pub async fn read_all(&self, pool: &MySqlPool) -> EntityResult<Vec<TestWithCourseChapter>> {
        let tests = sqlx::query_as::<_, TestWithCourseChapter>(
            r#"
            SELECT t.test_id, t.name as test_name, t.kind as test_kind, t.description as test_description, t.course_id, c.name as course_name, ch.chapter_id, ch.name as chapter_name
            FROM test t
            JOIN test_chapters tc ON t.test_id = tc.test_id
            JOIN chapter ch ON tc.chapter_id = ch.chapter_id
            JOIN course c ON t.course_id = c.course_id
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
    pub async fn read_by_hash(&self, pool: &MySqlPool) -> EntityResult<Option<TestEntity>> {
        let test = sqlx::query_as::<_, TestEntity>("SELECT * FROM test WHERE id_hash = ?").bind(&self.id_hash).fetch_optional(pool).await;
        match test {
            Ok(result) => EntityResult::Success(result),
            Err(e) => EntityResult::Error(DatabaseErrorType::QueryError("Failed to read test by hash".to_string(), e.to_string())),
        }
    }

    // get all tests in a course
    // capture test_id, id_hash, name, kind, course_id, course_name, added_timestamp, description in a new struct
    // then return the struct as a vector
    pub async fn read_by_course(&self, pool: &MySqlPool, course_id_hash: String) -> EntityResult<Vec<TestWithCourseChapter>> {
        let tests = sqlx::query_as::<_, TestWithCourseChapter>(
            r#"
            SELECT t.test_id, t.id_hash, t.name, t.kind, t.course_id, c.name as course_name, t.added_timestamp, t.description
            FROM test t
            JOIN course c ON t.course_id = c.course_id
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
    pub async fn read_by_chapter(&self, pool: &MySqlPool, chapter_id_hash: String) -> EntityResult<Vec<TestWithCourseChapter>> {
        let tests = sqlx::query_as::<_, TestWithCourseChapter>(
            r#"
            SELECT t.test_id, t.name as test_name, t.kind as test_kind, t.description as test_description, t.course_id, c.name as course_name, ch.chapter_id, ch.name as chapter_name
            FROM test t
            JOIN test_chapters tc ON t.test_id = tc.test_id
            JOIN chapter ch ON tc.chapter_id = ch.chapter_id
            JOIN course c ON t.course_id = c.course_id
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
