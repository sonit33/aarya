use sqlx::Error;
use sqlx::mysql::{MySqlPool, MySqlQueryResult};

#[derive(Debug, sqlx::FromRow)]
pub struct Feedback {
	pub feedback_id: i32,
	pub course_id: Option<i32>,
	pub test_id: Option<i32>,
	pub question_id: Option<i32>,
	pub teacher_id: Option<i32>,
	pub student_id: Option<i32>,
}

impl Feedback {
	pub async fn create(pool: &MySqlPool, course_id: Option<i32>, test_id: Option<i32>, question_id: Option<i32>, teacher_id: Option<i32>, student_id: Option<i32>) -> Result<MySqlQueryResult, Error> {
		let res = sqlx::query("INSERT INTO aarya_v1.feedbacks (course_id, test_id, question_id, teacher_id, student_id) VALUES (?, ?, ?, ?, ?)")
			.bind(course_id)
			.bind(test_id)
			.bind(question_id)
			.bind(teacher_id)
			.bind(student_id)
			.execute(pool)
			.await;
		match res {
			Ok(result) => Ok(result),
			Err(e) => Err(e),
		}
	}

	pub async fn read(pool: &MySqlPool, feedback_id: i32) -> Result<Option<Feedback>, Error> {
		let feedback = sqlx::query_as::<_, Feedback>("SELECT * FROM aarya_v1.feedbacks WHERE feedback_id = ?")
			.bind(feedback_id)
			.fetch_optional(pool)
			.await;
		match feedback {
			Ok(result) => Ok(result),
			Err(e) => Err(e),
		}
	}

	pub async fn update(pool: &MySqlPool, feedback_id: i32, course_id: Option<i32>, test_id: Option<i32>, question_id: Option<i32>, teacher_id: Option<i32>, student_id: Option<i32>) -> Result<MySqlQueryResult, Error> {
		let res = sqlx::query("UPDATE aarya_v1.feedbacks SET course_id = ?, test_id = ?, question_id = ?, teacher_id = ?, student_id = ? WHERE feedback_id = ?")
			.bind(course_id)
			.bind(test_id)
			.bind(question_id)
			.bind(teacher_id)
			.bind(student_id)
			.bind(feedback_id)
			.execute(pool)
			.await;
		match res {
			Ok(result) => Ok(result),
			Err(e) => Err(e),
		}
	}

	pub async fn delete(pool: &MySqlPool, feedback_id: i32) -> Result<MySqlQueryResult, Error> {
		let res = sqlx::query("DELETE FROM aarya_v1.feedbacks WHERE feedback_id = ?")
			.bind(feedback_id)
			.execute(pool)
			.await;
		match res {
			Ok(result) => Ok(result),
			Err(e) => Err(e),
		}
	}
}
