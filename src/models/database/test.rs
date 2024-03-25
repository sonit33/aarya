use sqlx::{Error, MySqlPool};
use sqlx::mysql::MySqlQueryResult;
use time::OffsetDateTime;

#[derive(Debug, sqlx::FromRow)]
pub struct Test {
	pub test_id: i32,
	pub course_id: i32,
	pub test_name: String,
	pub added_timestamp: Option<OffsetDateTime>,
	pub updated_timestamp: Option<OffsetDateTime>,
	pub mode: i8,
	pub test_size: i8,
}

impl Test {
	pub async fn create(
		pool: &MySqlPool,
		course_id: i32,
		test_name: &str,
		mode: i8,
		test_size: i8,
	) -> Result<MySqlQueryResult, Error> {
		let res = sqlx::query(
			"INSERT INTO tests (course_id, test_name, mode, test_size) VALUES (?, ?, ?, ?)",
		)
			.bind(course_id)
			.bind(test_name)
			.bind(mode)
			.bind(test_size)
			.execute(pool)
			.await;
		match res {
			Ok(result) => Ok(result),
			Err(e) => Err(e),
		}
	}

	pub async fn read(pool: &MySqlPool, test_id: i32) -> Result<Option<Test>, Error> {
		let test = sqlx::query_as::<_, Test>(
			"SELECT * FROM tests WHERE test_id = ?",
		)
			.bind(test_id)
			.fetch_optional(pool)
			.await;
		match test {
			Ok(result) => Ok(result),
			Err(e) => Err(e),
		}
	}

	pub async fn update(
		pool: &MySqlPool,
		test_id: i32,
		course_id: i32,
		test_name: &str,
		mode: i8,
		test_size: i8,
	) -> Result<MySqlQueryResult, Error> {
		let res = sqlx::query(
			"UPDATE tests SET course_id = ?, test_name = ?, mode = ?, test_size = ? WHERE test_id = ?",
		)
			.bind(course_id)
			.bind(test_name)
			.bind(mode)
			.bind(test_size)
			.bind(test_id)
			.execute(pool)
			.await;
		match res {
			Ok(result) => Ok(result),
			Err(e) => Err(e),
		}
	}

	pub async fn delete(pool: &MySqlPool, test_id: i32) -> Result<MySqlQueryResult, Error> {
		let res = sqlx::query(
			"DELETE FROM tests WHERE test_id = ?",
		)
			.bind(test_id)
			.execute(pool)
			.await;
		match res {
			Ok(result) => Ok(result),
			Err(e) => Err(e),
		}
	}
}
