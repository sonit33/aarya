use sqlx::{Error, MySqlPool};
use sqlx::mysql::MySqlQueryResult;

#[derive(Debug, sqlx::FromRow)]
pub struct QuestionTopic {
	pub question_id: i32,
	pub topic_id: i32,
}

impl QuestionTopic {
	pub async fn create(pool: &MySqlPool, question_id: i32, topic_id: i32) -> Result<MySqlQueryResult, Error> {
		let res = sqlx::query(
			"INSERT INTO question_topics (question_id, topic_id) VALUES (?, ?)",
		)
			.bind(question_id)
			.bind(topic_id)
			.execute(pool)
			.await;
		match res {
			Ok(result) => Ok(result),
			Err(e) => Err(e),
		}
	}

	pub async fn read(pool: &MySqlPool, question_id: i32, topic_id: i32) -> Result<Option<QuestionTopic>, Error> {
		let question_topic = sqlx::query_as::<_, QuestionTopic>(
			"SELECT * FROM question_topics WHERE question_id = ? AND topic_id = ?",
		)
			.bind(question_id)
			.bind(topic_id)
			.fetch_optional(pool)
			.await;
		match question_topic {
			Ok(result) => Ok(result),
			Err(e) => Err(e),
		}
	}

	pub async fn delete(pool: &MySqlPool, question_id: i32, topic_id: i32) -> Result<MySqlQueryResult, Error> {
		let res = sqlx::query(
			"DELETE FROM question_topics WHERE question_id = ? AND topic_id = ?",
		)
			.bind(question_id)
			.bind(topic_id)
			.execute(pool)
			.await;
		match res {
			Ok(result) => Ok(result),
			Err(e) => Err(e),
		}
	}
}
