use sqlx::{Error, MySqlPool};
use sqlx::mysql::MySqlQueryResult;

#[derive(Debug, sqlx::FromRow)]
pub struct TestTopic {
	pub test_id: i32,
	pub topic_id: i32,
}

impl TestTopic {
	pub async fn create(pool: &MySqlPool, test_id: i32, topic_id: i32) -> Result<MySqlQueryResult, Error> {
		let res = sqlx::query(
			"INSERT INTO test_topics (test_id, topic_id) VALUES (?, ?)",
		)
			.bind(test_id)
			.bind(topic_id)
			.execute(pool)
			.await;
		match res {
			Ok(result) => Ok(result),
			Err(e) => Err(e),
		}
	}

	pub async fn read(pool: &MySqlPool, test_id: i32, topic_id: i32) -> Result<Option<TestTopic>, Error> {
		let test_topic = sqlx::query_as::<_, TestTopic>(
			"SELECT * FROM test_topics WHERE test_id = ? AND topic_id = ?",
		)
			.bind(test_id)
			.bind(topic_id)
			.fetch_optional(pool)
			.await;
		match test_topic {
			Ok(result) => Ok(result),
			Err(e) => Err(e),
		}
	}

	pub async fn delete(pool: &MySqlPool, test_id: i32, topic_id: i32) -> Result<MySqlQueryResult, Error> {
		let res = sqlx::query(
			"DELETE FROM test_topics WHERE test_id = ? AND topic_id = ?",
		)
			.bind(test_id)
			.bind(topic_id)
			.execute(pool)
			.await;
		match res {
			Ok(result) => Ok(result),
			Err(e) => Err(e),
		}
	}
}
