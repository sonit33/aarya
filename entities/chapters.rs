use aarya_utils::hash_ops::string_hasher;
use serde::{Deserialize, Serialize};
use sqlx::MySqlPool;
use validator::Validate;

use crate::result_types::{DatabaseErrorType, EntityResult, SuccessResultType};

#[derive(Validate, Debug, Serialize, Deserialize, PartialEq, Clone, sqlx::FromRow)]
pub struct ChapterEntity {
    pub chapter_id: Option<u32>,
    pub course_id: Option<u32>,
    pub chapter_name: Option<String>,
    pub chapter_description: Option<String>,
}

#[derive(Validate, Debug, Serialize, Deserialize, PartialEq, Clone, sqlx::FromRow)]
pub struct ChapterQueryModel {
    pub chapter_id: u32,
    pub chapter_name: String,
    pub chapter_description: String,
    pub course_name: Option<String>,
}

impl ChapterEntity {
    pub fn new() -> Self {
        ChapterEntity {
            chapter_id: Some(0),
            course_id: None,
            chapter_name: None,
            chapter_description: None,
        }
    }

    pub async fn create_chapter(&self, pool: &MySqlPool) -> EntityResult<SuccessResultType> {
        let course_id = self.course_id;
        let name = self.chapter_name.clone().unwrap();
        let description = self.chapter_description.clone();
        let chapter_name_hash = string_hasher(&name);

        let query = r#"
            INSERT INTO chapter (course_id, chapter_name, chapter_description, chapter_name_hash)
            VALUES (?, ?, ?, ?)
        "#;

        match sqlx::query(query).bind(course_id).bind(name).bind(description).bind(chapter_name_hash).execute(pool).await {
            Ok(r) => EntityResult::Success(SuccessResultType::Created(r.last_insert_id(), r.rows_affected())),
            Err(e) => EntityResult::Error(DatabaseErrorType::QueryError("Error creating chapter".to_string(), e.to_string())),
        }
    }

    // get all chapters by joining with the course table to get course and chapter details incluidng course name
    pub async fn find_by_course(&self, pool: &MySqlPool) -> EntityResult<Vec<ChapterQueryModel>> {
        let query = r#"
            SELECT
                chapter_id,
                chapter_name,
                chapter_description,
                co.course_name,
                co.course_id
            FROM chapters ch
                JOIN courses co ON ch.course_id = co.course_id
            WHERE ch.course_id = ?
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
