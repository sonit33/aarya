use sqlx::Error;
use sqlx::mysql::{MySqlPool, MySqlQueryResult};

#[derive(Debug, sqlx::FromRow)]
pub struct Course {
	pub course_id: i32,
	pub course_name: String,
	pub added_timestamp: Option<time::OffsetDateTime>,
	pub updated_timestamp: Option<time::OffsetDateTime>,
	pub description: Option<String>,
}

impl Course {
	pub async fn create(pool: &MySqlPool, course_name: &str, description: Option<&str>) -> Result<MySqlQueryResult, Error> {
		let res = sqlx::query("INSERT INTO courses (course_name, description) VALUES (?, ?)")
			.bind(course_name)
			.bind(description)
			.execute(pool)
			.await;
		match res {
			Ok(result) => { Ok(result) }
			Err(e) => Err(e)
		}
	}

	pub async fn read(pool: &MySqlPool, course_id: i32) -> Result<Option<Course>, Error> {
		let course = sqlx::query_as::<_, Course>("SELECT * FROM courses WHERE course_id = ?")
			.bind(course_id)
			.fetch_optional(pool)
			.await;
		match course {
			Ok(result) => { Ok(result) }
			Err(e) => Err(e)
		}
	}

	pub async fn update(pool: &MySqlPool, course_id: i32, course_name: &str, description: Option<&str>) -> Result<MySqlQueryResult, Error> {
		let res = sqlx::query("UPDATE courses SET course_name = ?, description = ? WHERE course_id = ?")
			.bind(course_name)
			.bind(description)
			.bind(course_id)
			.execute(pool)
			.await;
		match res {
			Ok(result) => { Ok(result) }
			Err(e) => Err(e)
		}
	}

	pub async fn delete(pool: &MySqlPool, course_id: i32) -> Result<MySqlQueryResult, Error> {
		let res = sqlx::query("DELETE FROM courses WHERE course_id = ?")
			.bind(course_id)
			.execute(pool)
			.await;
		match res {
			Ok(result) => { Ok(result) }
			Err(e) => Err(e)
		}
	}
}
