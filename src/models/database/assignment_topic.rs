use sqlx::{Error, MySqlPool};
use sqlx::mysql::MySqlQueryResult;

#[derive(Debug, sqlx::FromRow)]
pub struct AssignmentTopic {
	pub assignment_id: i32,
	pub topic_id: i32,
}

impl AssignmentTopic {
	pub async fn create(pool: &MySqlPool, assignment_id: i32, topic_id: i32) -> Result<MySqlQueryResult, Error> {
		let res = sqlx::query(
			"INSERT INTO assignment_topics (assignment_id, topic_id) VALUES (?, ?)",
		)
			.bind(assignment_id)
			.bind(topic_id)
			.execute(pool)
			.await;
		match res {
			Ok(result) => Ok(result),
			Err(e) => Err(e),
		}
	}

	pub async fn read(pool: &MySqlPool, assignment_id: i32, topic_id: i32) -> Result<Option<AssignmentTopic>, Error> {
		let assignment_topic = sqlx::query_as::<_, AssignmentTopic>(
			"SELECT * FROM assignment_topics WHERE assignment_id = ? AND topic_id = ?",
		)
			.bind(assignment_id)
			.bind(topic_id)
			.fetch_optional(pool)
			.await;
		match assignment_topic {
			Ok(result) => Ok(result),
			Err(e) => Err(e),
		}
	}

	pub async fn delete(pool: &MySqlPool, assignment_id: i32, topic_id: i32) -> Result<MySqlQueryResult, Error> {
		let res = sqlx::query(
			"DELETE FROM assignment_topics WHERE assignment_id = ? AND topic_id = ?",
		)
			.bind(assignment_id)
			.bind(topic_id)
			.execute(pool)
			.await;
		match res {
			Ok(result) => Ok(result),
			Err(e) => Err(e),
		}
	}
}
