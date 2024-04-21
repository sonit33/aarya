use serde::{Deserialize, Serialize};
use sqlx::MySqlPool;
use validator::Validate;

use crate::result_types::{DatabaseErrorType, EntityResult, SuccessResultType};

#[derive(Validate, Debug, Serialize, Deserialize, PartialEq, Clone, sqlx::FromRow)]
pub struct TopicEntity {
    pub topic_id: Option<u32>,
    pub chapter_id: Option<u32>,
    pub course_id: Option<u32>,
    pub name: String,
    pub description: String,
}

#[derive(Validate, Debug, Serialize, Deserialize, PartialEq, Clone, sqlx::FromRow)]
pub struct TopicQueryModel {
    pub topic_id: u32,
    pub chapter_id: u32,
    pub course_id: u32,
    pub name: String,
    pub description: String,
    pub course_name: Option<String>,
    pub chapter_name: Option<String>,
}

impl TopicEntity {
    pub fn new() -> Self {
        TopicEntity {
            topic_id: Some(0),
            chapter_id: Some(0),
            course_id: Some(0),
            name: "not-set".to_string(),
            description: "not-set".to_string(),
        }
    }

    pub async fn create_topic(&self, pool: &MySqlPool) -> EntityResult<SuccessResultType> {
        let query = r#"
            INSERT INTO topic (course_id, chapter_id, name, description)
            VALUES (:course_id, :chapter_id, :name, :description)
        "#;

        match sqlx::query(query).bind(self.course_id).bind(&self.name).bind(&self.description).execute(pool).await {
            Ok(r) => EntityResult::Success(SuccessResultType::Created(r.last_insert_id(), r.rows_affected())),
            Err(e) => EntityResult::Error(DatabaseErrorType::QueryError("Error creating chapter".to_string(), e.to_string())),
        }
    }

    // get all chapters by joining with the course table to get course and chapter details incluidng course name
    pub async fn find(&self, pool: &MySqlPool) -> EntityResult<Vec<TopicQueryModel>> {
        let query = r#"
            SELECT
                topic_id,
                course_id,
                chapter_id,
                name,
                description,
                co.name as course_name,
                ch.name as chapter_name
            FROM topics t
                JOIN chapters ch ON t.chapter_id = ch.chapter_id
                JOIN courses co ON t.course_id = co.course_id
            where t.course_id = :course_id and t.chapter_id = :chapter_id
        "#;

        match sqlx::query_as::<_, TopicQueryModel>(query)
            .bind(self.course_id.unwrap())
            .bind(self.chapter_id.unwrap())
            .fetch_all(pool)
            .await
        {
            Ok(chapters) => EntityResult::Success(chapters),
            Err(e) => EntityResult::Error(DatabaseErrorType::QueryError("Error fetching chapters".to_string(), e.to_string())),
        }
    }
}

impl Default for TopicEntity {
    fn default() -> Self {
        Self::new()
    }
}
