use sqlx::{Error, MySqlPool};
use sqlx::mysql::MySqlQueryResult;

#[derive(Debug, sqlx::FromRow)]
pub struct TestStudent {
	pub test_id: i32,
	pub student_id: i32,
	pub ai_feedback: Option<String>,
}

impl TestStudent {
	pub async fn create(pool: &MySqlPool, test_id: i32, student_id: i32, ai_feedback: Option<&str>) -> Result<MySqlQueryResult, Error> {
		let res = sqlx::query(
			"INSERT INTO test_students (test_id, student_id, ai_feedback) VALUES (?, ?, ?)",
		)
			.bind(test_id)
			.bind(student_id)
			.bind(ai_feedback)
			.execute(pool)
			.await;
		match res {
			Ok(result) => Ok(result),
			Err(e) => Err(e),
		}
	}

	pub async fn read(pool: &MySqlPool, test_id: i32, student_id: i32) -> Result<Option<TestStudent>, Error> {
		let test_student = sqlx::query_as::<_, TestStudent>(
			"SELECT * FROM test_students WHERE test_id = ? AND student_id = ?",
		)
			.bind(test_id)
			.bind(student_id)
			.fetch_optional(pool)
			.await;
		match test_student {
			Ok(result) => Ok(result),
			Err(e) => Err(e),
		}
	}

	pub async fn update(pool: &MySqlPool, test_id: i32, student_id: i32, ai_feedback: Option<&str>) -> Result<MySqlQueryResult, Error> {
		let res = sqlx::query(
			"UPDATE test_students SET ai_feedback = ? WHERE test_id = ? AND student_id = ?",
		)
			.bind(ai_feedback)
			.bind(test_id)
			.bind(student_id)
			.execute(pool)
			.await;
		match res {
			Ok(result) => Ok(result),
			Err(e) => Err(e),
		}
	}

	pub async fn delete(pool: &MySqlPool, test_id: i32, student_id: i32) -> Result<MySqlQueryResult, Error> {
		let res = sqlx::query(
			"DELETE FROM test_students WHERE test_id = ? AND student_id = ?",
		)
			.bind(test_id)
			.bind(student_id)
			.execute(pool)
			.await;
		match res {
			Ok(result) => Ok(result),
			Err(e) => Err(e),
		}
	}
}
