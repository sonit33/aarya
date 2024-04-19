use sqlx::MySqlPool;

use super::result_type::{DatabaseErrorType, EntityResult, SuccessResultType};

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
