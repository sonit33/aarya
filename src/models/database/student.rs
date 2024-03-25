use sqlx::{Error, MySqlPool};
use sqlx::mysql::MySqlQueryResult;

#[derive(Debug, sqlx::FromRow)]
pub struct Student {
	pub student_id: i32,
	pub first_name: String,
	pub email_address: String,
	pub password: String,
	pub over_13: i8,
	pub email_verified: i8,
	pub account_active: i8,
	pub added_timestamp: Option<time::OffsetDateTime>,
	pub updated_timestamp: Option<time::OffsetDateTime>,
	pub deleted_timestamp: Option<time::OffsetDateTime>,
}

impl Student {
	pub async fn create(
		pool: &MySqlPool,
		first_name: &str,
		email_address: &str,
		password: &str,
		over_13: i8,
		email_verified: i8,
		account_active: i8,
	) -> Result<MySqlQueryResult, Error> {
		let res = sqlx::query(
			"INSERT INTO students (first_name, email_address, password, over_13, email_verified, account_active) VALUES (?, ?, ?, ?, ?, ?)",
		)
			.bind(first_name)
			.bind(email_address)
			.bind(password)
			.bind(over_13)
			.bind(email_verified)
			.bind(account_active)
			.execute(pool)
			.await;
		match res {
			Ok(result) => Ok(result),
			Err(e) => Err(e),
		}
	}

	pub async fn read(pool: &MySqlPool, student_id: i32) -> Result<Option<Student>, Error> {
		let student = sqlx::query_as::<_, Student>(
			"SELECT * FROM students WHERE student_id = ?",
		)
			.bind(student_id)
			.fetch_optional(pool)
			.await;
		match student {
			Ok(result) => Ok(result),
			Err(e) => Err(e),
		}
	}

	pub async fn update(
		pool: &MySqlPool,
		student_id: i32,
		first_name: &str,
		email_address: &str,
		password: &str,
		over_13: i8,
		email_verified: i8,
		account_active: i8,
	) -> Result<MySqlQueryResult, Error> {
		let res = sqlx::query(
			"UPDATE students SET first_name = ?, email_address = ?, password = ?, over_13 = ?, email_verified = ?, account_active = ? WHERE student_id = ?",
		)
			.bind(first_name)
			.bind(email_address)
			.bind(password)
			.bind(over_13)
			.bind(email_verified)
			.bind(account_active)
			.bind(student_id)
			.execute(pool)
			.await;
		match res {
			Ok(result) => Ok(result),
			Err(e) => Err(e),
		}
	}

	pub async fn delete(pool: &MySqlPool, student_id: i32) -> Result<MySqlQueryResult, Error> {
		let res = sqlx::query(
			"UPDATE students SET deleted_timestamp = CURRENT_TIMESTAMP WHERE student_id = ?",
		)
			.bind(student_id)
			.execute(pool)
			.await;
		match res {
			Ok(result) => Ok(result),
			Err(e) => Err(e),
		}
	}
}
