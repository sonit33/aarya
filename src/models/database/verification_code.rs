use sqlx::{Error, MySqlPool};
use sqlx::mysql::MySqlQueryResult;
use time::OffsetDateTime;

#[derive(Debug, sqlx::FromRow)]
pub struct VerificationCode {
	pub code: String,
	pub student_id: Option<i32>,
	pub teacher_id: Option<i32>,
	pub added_timestamp: Option<OffsetDateTime>,
	pub updated_timestamp: Option<OffsetDateTime>,
}

impl VerificationCode {
	pub async fn create_or_update_student_code(pool: &MySqlPool, student_id: i32, code: &str) -> Result<MySqlQueryResult, Error> {
		let res = sqlx::query(
			"INSERT INTO verification_codes (code, student_id, teacher_id) VALUES (?, ?, 0)
             ON DUPLICATE KEY UPDATE code = ?",
		)
			.bind(code)
			.bind(student_id)
			.bind(code)
			.execute(pool)
			.await;
		res
	}

	pub async fn create_or_update_teacher_code(pool: &MySqlPool, teacher_id: i32, code: &str) -> Result<MySqlQueryResult, Error> {
		let res = sqlx::query(
			"INSERT INTO verification_codes (code, student_id, teacher_id) VALUES (?, 0, ?)
             ON DUPLICATE KEY UPDATE code = ?",
		)
			.bind(code)
			.bind(teacher_id)
			.bind(code)
			.execute(pool)
			.await;
		res
	}

	pub async fn read_student_code(pool: &MySqlPool, student_id: i32) -> Result<Option<VerificationCode>, Error> {
		let verification_code = sqlx::query_as::<_, VerificationCode>(
			"SELECT * FROM verification_codes WHERE student_id = ?",
		)
			.bind(student_id)
			.fetch_optional(pool)
			.await;
		verification_code
	}

	pub async fn read_teacher_code(pool: &MySqlPool, teacher_id: i32) -> Result<Option<VerificationCode>, Error> {
		let verification_code = sqlx::query_as::<_, VerificationCode>(
			"SELECT * FROM verification_codes WHERE teacher_id = ?",
		)
			.bind(teacher_id)
			.fetch_optional(pool)
			.await;
		verification_code
	}
}
