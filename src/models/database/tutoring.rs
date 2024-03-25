use sqlx::{Error, MySqlPool, };
use sqlx::mysql::MySqlQueryResult;

#[derive(Debug, sqlx::FromRow)]
pub struct Tutoring {
	pub tutoring_id: i32,
	pub course_id: i32,
	pub student_id: i32,
	pub teacher_id: i32,
	pub t_type: i8,
}

impl Tutoring {
	pub async fn create(
		pool: &MySqlPool,
		course_id: i32,
		student_id: i32,
		teacher_id: i32,
		t_type: i8,
	) -> Result<MySqlQueryResult, Error> {
		let res = sqlx::query(
			"INSERT INTO tutoring (course_id, student_id, teacher_id, t_type) VALUES (?, ?, ?, ?)",
		)
			.bind(course_id)
			.bind(student_id)
			.bind(teacher_id)
			.bind(t_type)
			.execute(pool)
			.await;
		match res {
			Ok(result) => Ok(result),
			Err(e) => Err(e),
		}
	}

	pub async fn read(pool: &MySqlPool, tutoring_id: i32) -> Result<Option<Tutoring>, Error> {
		let tutoring = sqlx::query_as::<_, Tutoring>(
			"SELECT * FROM tutoring WHERE tutoring_id = ?",
		)
			.bind(tutoring_id)
			.fetch_optional(pool)
			.await;
		match tutoring {
			Ok(result) => Ok(result),
			Err(e) => Err(e),
		}
	}

	pub async fn update(
		pool: &MySqlPool,
		tutoring_id: i32,
		course_id: i32,
		student_id: i32,
		teacher_id: i32,
		t_type: i8,
	) -> Result<MySqlQueryResult, Error> {
		let res = sqlx::query(
			"UPDATE tutoring SET course_id = ?, student_id = ?, teacher_id = ?, t_type = ? WHERE tutoring_id = ?",
		)
			.bind(course_id)
			.bind(student_id)
			.bind(teacher_id)
			.bind(t_type)
			.bind(tutoring_id)
			.execute(pool)
			.await;
		match res {
			Ok(result) => Ok(result),
			Err(e) => Err(e),
		}
	}

	pub async fn delete(pool: &MySqlPool, tutoring_id: i32) -> Result<MySqlQueryResult, Error> {
		let res = sqlx::query(
			"DELETE FROM tutoring WHERE tutoring_id = ?",
		)
			.bind(tutoring_id)
			.execute(pool)
			.await;
		match res {
			Ok(result) => Ok(result),
			Err(e) => Err(e),
		}
	}
}
