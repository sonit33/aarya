use sqlx::MySqlPool;
use time::OffsetDateTime;

use super::result_type::{DatabaseErrorType, EntityResult, SuccessResultType};

#[derive(Debug, sqlx::FromRow)]
pub struct Course {
    pub course_id: u32,
    pub name: String,
    pub id_hash: String,
    pub added_timestamp: Option<OffsetDateTime>,
    pub updated_timestamp: Option<OffsetDateTime>,
    pub description: String,
}

impl Course {
    pub fn new() -> Self {
        Course {
            course_id: 0,
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
            INSERT INTO course (name, id_hash, description)
            VALUES (?, ?, ?)
        "#;

        match sqlx::query(query).bind(name).bind(id_hash).bind(description).execute(pool).await {
            Ok(_) => EntityResult::Success(SuccessResultType::Created(0, 0)),
            Err(e) => EntityResult::Error(DatabaseErrorType::QueryError("Error creating course".to_string(), e.to_string())),
        }
    }

    // get all courses
    pub async fn get_courses(pool: &MySqlPool) -> EntityResult<Vec<Course>> {
        let query = r#"
            SELECT course_id, name, id_hash, description
            FROM course
        "#;

        match sqlx::query_as::<_, Course>(query).fetch_all(pool).await {
            Ok(courses) => EntityResult::Success(courses),
            Err(e) => EntityResult::Error(DatabaseErrorType::QueryError("Error fetching courses".to_string(), e.to_string())),
        }
    }
}

impl Default for Course {
    fn default() -> Self {
        Self::new()
    }
}
