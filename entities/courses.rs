use aarya_utils::hash_ops::string_hasher;
use serde::{Deserialize, Serialize};
use sqlx::MySqlPool;
use validator::Validate;

use crate::result_types::{DatabaseErrorType, EntityResult, SuccessResultType};

#[derive(Validate, Debug, Serialize, Deserialize, PartialEq, Clone, sqlx::FromRow)]
pub struct CourseEntity {
    pub course_id: u32,
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
            course_id: 0,
            course_name: "not-set".to_string(),
            course_description: "not-set".to_string(),
        }
    }

    // create a new course
    pub async fn create(
        &self,
        pool: &MySqlPool,
    ) -> EntityResult<SuccessResultType> {
        let name = self.course_name.clone();
        let description = self.course_description.clone();
        let course_name_hash = string_hasher(&name);

        let query = r#"
            INSERT INTO courses (course_id, course_name, course_description, course_name_hash)
            VALUES (?, ?, ?, ?)
        "#;

        match sqlx::query(query).bind(self.course_id).bind(name).bind(description).bind(course_name_hash).execute(pool).await {
            Ok(d) => EntityResult::Success(SuccessResultType::Created(self.course_id as u64, d.rows_affected())),
            Err(e) => EntityResult::Error(DatabaseErrorType::QueryError("Error creating course".to_string(), e.to_string())),
        }
    }

    // get all courses
    pub async fn find_all(
        &self,
        pool: &MySqlPool,
    ) -> EntityResult<Vec<CourseQueryModel>> {
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
