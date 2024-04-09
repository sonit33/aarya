use sqlx::mysql::{MySqlPool, MySqlQueryResult};
use sqlx::Error;

#[derive(Debug, sqlx::FromRow)]
pub struct Course {
    pub course_id: Option<u32>,
    pub name: String,
    pub added_timestamp: Option<time::OffsetDateTime>,
    pub updated_timestamp: Option<time::OffsetDateTime>,
    pub description: String
}

impl Course {
    pub fn new() -> Self {
        Course {
            course_id: Some(0),
            name: String::from("not-set"),
            added_timestamp: None,
            updated_timestamp: None,
            description: String::from("not-set")
        }
    }
}

impl Course {
    pub async fn create(&self, pool: &MySqlPool) -> Result<MySqlQueryResult, Error> {
        let res = sqlx::query("INSERT INTO courses (name, description) VALUES (?, ?)")
            .bind(&self.name)
            .bind(&self.description)
            .execute(pool)
            .await;
        match res {
            Ok(result) => Ok(result),
            Err(e) => Err(e)
        }
    }

    pub async fn read(&self, pool: &MySqlPool) -> Result<Option<Course>, Error> {
        let course = sqlx::query_as::<_, Course>("SELECT * FROM courses WHERE course_id = ?")
            .bind(&self.course_id)
            .fetch_optional(pool)
            .await;
        match course {
            Ok(result) => Ok(result),
            Err(e) => Err(e)
        }
    }

    pub async fn update(&self, pool: &MySqlPool) -> Result<MySqlQueryResult, Error> {
        let res = sqlx::query("UPDATE courses SET name = ?, description = ? WHERE course_id = ?")
            .bind(&self.name)
            .bind(&self.description)
            .bind(&self.course_id)
            .execute(pool)
            .await;
        match res {
            Ok(result) => Ok(result),
            Err(e) => Err(e)
        }
    }

    pub async fn delete(&self, pool: &MySqlPool) -> Result<MySqlQueryResult, Error> {
        let res = sqlx::query("DELETE FROM courses WHERE course_id = ?").bind(&self.course_id).execute(pool).await;
        match res {
            Ok(result) => Ok(result),
            Err(e) => Err(e)
        }
    }
}
