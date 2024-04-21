use serde::{Deserialize, Serialize};
use sqlx::MySqlPool;
use validator::Validate;

use crate::result_types::{DatabaseErrorType, EntityResult, SuccessResultType};

#[derive(Validate, Debug, Serialize, Deserialize, PartialEq, Clone, sqlx::FromRow)]
pub struct ChapterEntity {
    pub chapter_id: Option<u32>,
    pub course_id: Option<u32>,
    pub name: Option<String>,
    pub description: Option<String>,
}

#[derive(Validate, Debug, Serialize, Deserialize, PartialEq, Clone, sqlx::FromRow)]
pub struct ChapterQueryModel {
    pub name: String,
    pub description: String,
    pub course_name: Option<String>,
}

impl ChapterEntity {
    pub fn new() -> Self {
        ChapterEntity {
            chapter_id: Some(0),
            course_id: None,
            name: None,
            description: None,
        }
    }

    pub async fn create_chapter(&self, pool: &MySqlPool) -> EntityResult<SuccessResultType> {
        let course_id = self.course_id;
        let name = self.name.clone();
        let description = self.description.clone();

        let query = r#"
            INSERT INTO chapter (course_id, name, description)
            VALUES (?, ?, ?)
        "#;

        match sqlx::query(query).bind(course_id).bind(name).bind(description).execute(pool).await {
            Ok(r) => EntityResult::Success(SuccessResultType::Created(r.last_insert_id(), r.rows_affected())),
            Err(e) => EntityResult::Error(DatabaseErrorType::QueryError("Error creating chapter".to_string(), e.to_string())),
        }
    }

    // get all chapters by joining with the course table to get course and chapter details incluidng course name
    pub async fn find_by_course(&self, pool: &MySqlPool) -> EntityResult<Vec<ChapterQueryModel>> {
        let query = r#"
            SELECT
                ch.name,
                ch.description,
                co.name as course_name,
                co.course_id as course_id
            FROM chapters ch
                JOIN courses co ON ch.course_id = co.course_id
            where ch.course_id = ?
        "#;

        match sqlx::query_as::<_, ChapterQueryModel>(query).bind(self.course_id.unwrap()).fetch_all(pool).await {
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
