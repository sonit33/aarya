use serde::{Deserialize, Serialize};
use sqlx::MySqlPool;
use validator::Validate;

use crate::result_types::{DatabaseErrorType, EntityResult, SuccessResultType};

#[derive(Validate, Debug, Serialize, Deserialize, PartialEq, Clone, sqlx::FromRow)]
pub struct CourseEntity {
    pub course_id: Option<u32>,
    pub course_name: String,
    pub course_description: String,
}

#[derive(Validate, Debug, Serialize, Deserialize, PartialEq, Clone, sqlx::FromRow)]
pub struct CourseQueryModel {
    pub id: u32,
    pub name: String,
    pub description: String,
}

impl CourseEntity {
    pub fn new() -> Self {
        CourseEntity {
            course_id: Some(0),
            course_name: "not-set".to_string(),
            course_description: "not-set".to_string(),
        }
    }

    // create a new course
    pub async fn create(&self, pool: &MySqlPool) -> EntityResult<SuccessResultType> {
        let name = self.course_name.clone();
        let description = self.course_description.clone();

        let query = r#"
            INSERT INTO courses (course_name, course_description)
            VALUES (?, ?)
        "#;

        match sqlx::query(query).bind(name).bind(description).execute(pool).await {
            Ok(_) => EntityResult::Success(SuccessResultType::Created(0, 0)),
            Err(e) => EntityResult::Error(DatabaseErrorType::QueryError("Error creating course".to_string(), e.to_string())),
        }
    }

    // get all courses
    pub async fn find_all(&self, pool: &MySqlPool) -> EntityResult<Vec<CourseQueryModel>> {
        let query = r#"
            SELECT course_id as id, course_name as name, course_description as description FROM courses
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
