use sqlx::{Error, MySqlPool};
use sqlx::mysql::MySqlQueryResult;
use time::OffsetDateTime;

#[derive(Debug, sqlx::FromRow)]
pub struct Topic {
	pub topic_id: i32,
	pub course_id: i32,
	pub topic_name: String,
	pub description: Option<String>,
	pub added_timestamp: Option<OffsetDateTime>,
	pub updated_timestamp: Option<OffsetDateTime>,
}

impl Topic {
	pub async fn create(
		pool: &MySqlPool,
		course_id: i32,
		topic_name: &str,
		description: Option<&str>,
	) -> Result<MySqlQueryResult, Error> {
		let res = sqlx::query(
			"INSERT INTO topics (course_id, topic_name, description) VALUES (?, ?, ?)",
		)
			.bind(course_id)
			.bind(topic_name)
			.bind(description)
			.execute(pool)
			.await;
		match res {
			Ok(result) => Ok(result),
			Err(e) => Err(e),
		}
	}

	pub async fn read(pool: &MySqlPool, topic_id: i32) -> Result<Option<Topic>, Error> {
		let topic = sqlx::query_as::<_, Topic>(
			"SELECT * FROM topics WHERE topic_id = ?",
		)
			.bind(topic_id)
			.fetch_optional(pool)
			.await;
		match topic {
			Ok(result) => Ok(result),
			Err(e) => Err(e),
		}
	}

	pub async fn update(
		pool: &MySqlPool,
		topic_id: i32,
		course_id: i32,
		topic_name: &str,
		description: Option<&str>,
	) -> Result<MySqlQueryResult, Error> {
		let res = sqlx::query(
			"UPDATE topics SET course_id = ?, topic_name = ?, description = ? WHERE topic_id = ?",
		)
			.bind(course_id)
			.bind(topic_name)
			.bind(description)
			.bind(topic_id)
			.execute(pool)
			.await;
		match res {
			Ok(result) => Ok(result),
			Err(e) => Err(e),
		}
	}

	pub async fn delete(pool: &MySqlPool, topic_id: i32) -> Result<MySqlQueryResult, Error> {
		let res = sqlx::query(
			"DELETE FROM topics WHERE topic_id = ?",
		)
			.bind(topic_id)
			.execute(pool)
			.await;
		match res {
			Ok(result) => Ok(result),
			Err(e) => Err(e),
		}
	}
}
