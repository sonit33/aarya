use sqlx::{Error, MySqlPool, };
use sqlx::mysql::MySqlQueryResult;
use time::OffsetDateTime;

#[derive(Debug, sqlx::FromRow)]
pub struct Payment {
	pub payment_id: i32,
	pub p_direction: i8,
	pub p_amount: f64,
	pub added_timestamp: Option<OffsetDateTime>,
	pub p_id: Option<i32>,
	pub student_id: Option<i32>,
	pub teacher_id: Option<i32>,
	pub purpose: i8,
}

impl Payment {
	pub async fn create(
		pool: &MySqlPool,
		p_direction: i8,
		p_amount: f64,
		p_id: Option<i32>,
		student_id: Option<i32>,
		teacher_id: Option<i32>,
		purpose: i8,
	) -> Result<MySqlQueryResult, Error> {
		let res = sqlx::query(
			"INSERT INTO payments (p_direction, p_amount, p_id, student_id, teacher_id, purpose) VALUES (?, ?, ?, ?, ?, ?)",
		)
			.bind(p_direction)
			.bind(p_amount)
			.bind(p_id)
			.bind(student_id)
			.bind(teacher_id)
			.bind(purpose)
			.execute(pool)
			.await;
		match res {
			Ok(result) => Ok(result),
			Err(e) => Err(e),
		}
	}

	pub async fn read(pool: &MySqlPool, payment_id: i32) -> Result<Option<Payment>, Error> {
		let payment = sqlx::query_as::<_, Payment>(
			"SELECT * FROM payments WHERE payment_id = ?",
		)
			.bind(payment_id)
			.fetch_optional(pool)
			.await;
		match payment {
			Ok(result) => Ok(result),
			Err(e) => Err(e),
		}
	}

	pub async fn update(
		pool: &MySqlPool,
		payment_id: i32,
		p_direction: i8,
		p_amount: f64,
		p_id: Option<i32>,
		student_id: Option<i32>,
		teacher_id: Option<i32>,
		purpose: i8,
	) -> Result<MySqlQueryResult, Error> {
		let res = sqlx::query(
			"UPDATE payments SET p_direction = ?, p_amount = ?, p_id = ?, student_id = ?, teacher_id = ?, purpose = ? WHERE payment_id = ?",
		)
			.bind(p_direction)
			.bind(p_amount)
			.bind(p_id)
			.bind(student_id)
			.bind(teacher_id)
			.bind(purpose)
			.bind(payment_id)
			.execute(pool)
			.await;
		match res {
			Ok(result) => Ok(result),
			Err(e) => Err(e),
		}
	}

	pub async fn delete(pool: &MySqlPool, payment_id: i32) -> Result<MySqlQueryResult, Error> {
		let res = sqlx::query(
			"DELETE FROM payments WHERE payment_id = ?",
		)
			.bind(payment_id)
			.execute(pool)
			.await;
		match res {
			Ok(result) => Ok(result),
			Err(e) => Err(e),
		}
	}
}
