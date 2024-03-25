use sqlx::{Error, MySqlPool};
use sqlx::mysql::MySqlQueryResult;

#[derive(Debug, sqlx::FromRow)]
pub struct TestQuestion {
	pub test_id: i32,
	pub question_id: i32,
}

impl TestQuestion {
	pub async fn create(pool: &MySqlPool, test_id: i32, question_id: i32) -> Result<MySqlQueryResult, Error> {
		let res = sqlx::query(
			"INSERT INTO test_questions (test_id, question_id) VALUES (?, ?)",
		)
			.bind(test_id)
			.bind(question_id)
			.execute(pool)
			.await;
		match res {
			Ok(result) => Ok(result),
			Err(e) => Err(e),
		}
	}

	// Read is typically not implemented for association tables, but provided for completeness
	pub async fn read(pool: &MySqlPool, test_id: i32, question_id: i32) -> Result<Option<TestQuestion>, Error> {
		let test_question = sqlx::query_as::<_, TestQuestion>(
			"SELECT * FROM test_questions WHERE test_id = ? AND question_id = ?",
		)
			.bind(test_id)
			.bind(question_id)
			.fetch_optional(pool)
			.await;
		match test_question {
			Ok(result) => Ok(result),
			Err(e) => Err(e),
		}
	}

	pub async fn delete(pool: &MySqlPool, test_id: i32, question_id: i32) -> Result<MySqlQueryResult, Error> {
		let res = sqlx::query(
			"DELETE FROM test_questions WHERE test_id = ? AND question_id = ?",
		)
			.bind(test_id)
			.bind(question_id)
			.execute(pool)
			.await;
		match res {
			Ok(result) => Ok(result),
			Err(e) => Err(e),
		}
	}
}
