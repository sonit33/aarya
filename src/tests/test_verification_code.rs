use sqlx::MySqlPool;

use crate::models::database::student::Student;
use crate::models::database::teacher::Teacher;
use crate::models::database::verification_code::VerificationCode;
use crate::tests::{setup_database, teardown_database};
use crate::utils::random::generate_guid;

// Function to create a student and a teacher to satisfy the foreign key constraints of the verification_codes table
async fn create_student_and_teacher(pool: &MySqlPool) -> (i32, i32) {
	let student_result = Student::create(pool, "John Doe", "john.doe@example.com", "password123", 1, 1, 1).await.unwrap();
	let student_id = student_result.last_insert_id() as i32;

	let teacher_result = Teacher::create(
		pool,
		"Jane Doe",
		Some("High School"),
		"securepassword",
		"jane.doe@example.com",
		Some("http://example.com/photo.jpg"),
		Some("Teaching high school math."),
		Some("Masters in Mathematics Education"),
		Some("Mathematics"),
		Some("Certified Mathematics Teacher"),
		Some("High School"),
		1,
		1,
		Some("@janevenmo"),
		Some("janepaypal@example.com"),
		Some(1)
	).await.unwrap();
	let teacher_id = teacher_result.last_insert_id() as i32;

	(student_id, teacher_id)
}

#[tokio::test]
async fn test_create_or_update_student_code() {
	let db_name = generate_guid(8);
	let pool = setup_database(&db_name).await;

	// Create a student to satisfy foreign key constraint
	let (student_id, _) = create_student_and_teacher(&pool).await;

	// Create or update a verification code for the student
	let code = "STUDENT1234";
	let result = VerificationCode::create_or_update_student_code(&pool, student_id, code).await;
	assert!(result.is_ok(), "Failed to read result: {:?}", result.err());

	// Verify the creation or update by reading the code back
	let read_result = VerificationCode::read_student_code(&pool, student_id).await;
	assert!(read_result.is_ok());
	let verification_code = read_result.unwrap().unwrap();
	assert_eq!(verification_code.code, code);

	teardown_database(&pool, &db_name).await.unwrap();
}

#[tokio::test]
async fn test_create_or_update_teacher_code() {
	let db_name = generate_guid(8);
	let pool = setup_database(&db_name).await;

	// Create a teacher to satisfy foreign key constraint
	let (_, teacher_id) = create_student_and_teacher(&pool).await;

	// Create or update a verification code for the teacher
	let code = "TEACHER1234";
	let result = VerificationCode::create_or_update_teacher_code(&pool, teacher_id, code).await;
	assert!(result.is_ok(), "Failed to read result: {:?}", result.err());

	// Verify the creation or update by reading the code back
	let read_result = VerificationCode::read_teacher_code(&pool, teacher_id).await;
	assert!(read_result.is_ok());
	let verification_code = read_result.unwrap().unwrap();
	assert_eq!(verification_code.code, code);

	teardown_database(&pool, &db_name).await.unwrap();
}
