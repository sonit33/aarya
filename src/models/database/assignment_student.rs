use sqlx::{Error, MySqlPool};
use sqlx::mysql::MySqlQueryResult;

#[derive(Debug, sqlx::FromRow)]
pub struct AssignmentStudent {
	pub assignment_id: i32,
	pub student_id: i32,
}

impl AssignmentStudent {
	pub async fn create(pool: &MySqlPool, assignment_id: i32, student_id: i32) -> Result<MySqlQueryResult, Error> {
		let res = sqlx::query(
			"INSERT INTO assignment_students (assignment_id, student_id) VALUES (?, ?)",
		)
			.bind(assignment_id)
			.bind(student_id)
			.execute(pool)
			.await;
		match res {
			Ok(result) => Ok(result),
			Err(e) => Err(e),
		}
	}

	// Read is not typical for many-to-many relationship tables, but provided for completeness
	pub async fn read(pool: &MySqlPool, assignment_id: i32, student_id: i32) -> Result<Option<AssignmentStudent>, Error> {
		let assignment_student = sqlx::query_as::<_, AssignmentStudent>(
			"SELECT * FROM assignment_students WHERE assignment_id = ? AND student_id = ?",
		)
			.bind(assignment_id)
			.bind(student_id)
			.fetch_optional(pool)
			.await;
		match assignment_student {
			Ok(result) => Ok(result),
			Err(e) => Err(e),
		}
	}

	// Deleting a specific association between an assignment and a student
	pub async fn delete(pool: &MySqlPool, assignment_id: i32, student_id: i32) -> Result<MySqlQueryResult, Error> {
		let res = sqlx::query(
			"DELETE FROM assignment_students WHERE assignment_id = ? AND student_id = ?",
		)
			.bind(assignment_id)
			.bind(student_id)
			.execute(pool)
			.await;
		match res {
			Ok(result) => Ok(result),
			Err(e) => Err(e),
		}
	}
}
