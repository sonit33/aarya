use serde::{Deserialize, Serialize};
use sqlx::MySqlPool;
use validator::Validate;

use crate::result_types::{DatabaseErrorType, EntityResult, SuccessResultType};

#[derive(Validate, Debug, Serialize, Deserialize, PartialEq, Clone, sqlx::FromRow)]
pub struct CourseEntity {
    pub course_id: Option<u32>,
    pub name: String,
    pub id_hash: String,
    pub description: String,
}

#[derive(Validate, Debug, Serialize, Deserialize, PartialEq, Clone, sqlx::FromRow)]
pub struct CourseQueryModel {
    pub name: String,
    pub id_hash: String,
    pub description: String,
}

impl CourseEntity {
    pub fn new() -> Self {
        CourseEntity {
            course_id: Some(0),
            name: "not-set".to_string(),
            id_hash: "not-set".to_string(),
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
    pub async fn find_all(&self, pool: &MySqlPool) -> EntityResult<Vec<CourseQueryModel>> {
        let query = r#"
            SELECT id_hash, name, description FROM courses
        "#;

        match sqlx::query_as::<_, CourseQueryModel>(query).fetch_all(pool).await {
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
