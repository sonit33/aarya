use sqlx::{Error, MySqlPool};
use sqlx::mysql::MySqlQueryResult;
use time::OffsetDateTime;

#[derive(Debug, sqlx::FromRow)]
pub struct Assignment {
	pub assignment_id: i32,
	pub course_id: i32,
	pub teacher_id: i32,
	pub assignment_name: String,
	pub added_timestamp: Option<OffsetDateTime>,
	pub due_on_timestamp: OffsetDateTime,
}

impl Assignment {
	pub async fn create(
		pool: &MySqlPool,
		course_id: i32,
		teacher_id: i32,
		assignment_name: &str,
		due_on_timestamp: OffsetDateTime,
	) -> Result<MySqlQueryResult, Error> {
		let res = sqlx::query(
			"INSERT INTO assignments (course_id, teacher_id, assignment_name, due_on_timestamp) VALUES (?, ?, ?, ?)",
		)
			.bind(course_id)
			.bind(teacher_id)
			.bind(assignment_name)
			.bind(due_on_timestamp)
			.execute(pool)
			.await;
		match res {
			Ok(result) => Ok(result),
			Err(e) => Err(e),
		}
	}

	pub async fn read(pool: &MySqlPool, assignment_id: i32) -> Result<Option<Assignment>, Error> {
		let assignment = sqlx::query_as::<_, Assignment>(
			"SELECT * FROM assignments WHERE assignment_id = ?",
		)
			.bind(assignment_id)
			.fetch_optional(pool)
			.await;
		match assignment {
			Ok(result) => Ok(result),
			Err(e) => Err(e),
		}
	}

	pub async fn update(
		pool: &MySqlPool,
		assignment_id: i32,
		course_id: i32,
		teacher_id: i32,
		assignment_name: &str,
		due_on_timestamp: OffsetDateTime,
	) -> Result<MySqlQueryResult, Error> {
		let res = sqlx::query(
			"UPDATE assignments SET course_id = ?, teacher_id = ?, assignment_name = ?, due_on_timestamp = ? WHERE assignment_id = ?",
		)
			.bind(course_id)
			.bind(teacher_id)
			.bind(assignment_name)
			.bind(due_on_timestamp)
			.bind(assignment_id)
			.execute(pool)
			.await;
		match res {
			Ok(result) => Ok(result),
			Err(e) => Err(e),
		}
	}

	pub async fn delete(pool: &MySqlPool, assignment_id: i32) -> Result<MySqlQueryResult, Error> {
		let res = sqlx::query(
			"DELETE FROM assignments WHERE assignment_id = ?",
		)
			.bind(assignment_id)
			.execute(pool)
			.await;
		match res {
			Ok(result) => Ok(result),
			Err(e) => Err(e),
		}
	}
}
