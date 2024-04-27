use aarya_utils::hash_ops::string_hasher;
use serde::{Deserialize, Serialize};
use sqlx::MySqlPool;
use validator::Validate;

use crate::result_types::{DatabaseErrorType, EntityResult, SuccessResultType};

#[derive(Validate, Debug, Serialize, Deserialize, PartialEq, Clone, sqlx::FromRow)]
pub struct ChapterEntity {
    pub chapter_id: u32,
    pub course_id: u32,
    pub chapter_name: String,
    pub chapter_description: String,
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
            chapter_id: 0,
            course_id: 0,
            chapter_name: "not-set".to_string(),
            chapter_description: "not-set".to_string(),
        }
    }

    pub async fn create_chapter(
        &self,
        pool: &MySqlPool,
    ) -> EntityResult<SuccessResultType> {
        // let course_id = self.course_id;
        // let name = self.chapter_name.clone().unwrap();
        // let description = self.chapter_description.clone();
        let chapter_name_hash = string_hasher(&self.chapter_name);

        let query = r#"
            INSERT INTO chapters (chapter_id, course_id, chapter_name, chapter_description, chapter_name_hash)
            VALUES (?, ?, ?, ?, ?)
        "#;

        match sqlx::query(query)
            .bind(self.chapter_id)
            .bind(self.course_id)
            .bind(&self.chapter_name)
            .bind(&self.chapter_description)
            .bind(chapter_name_hash)
            .execute(pool)
            .await
        {
            Ok(r) => EntityResult::Success(SuccessResultType::Created(self.chapter_id as u64, r.rows_affected())),
            Err(e) => EntityResult::Error(DatabaseErrorType::QueryError("Error creating chapter".to_string(), e.to_string())),
        }
    }

    // get all chapters by joining with the course table to get course and chapter details incluidng course name
    pub async fn find_by_course(
        &self,
        pool: &MySqlPool,
    ) -> EntityResult<Vec<ChapterQueryModel>> {
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

        match sqlx::query_as::<_, ChapterQueryModel>(query).bind(self.course_id).fetch_all(pool).await {
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
