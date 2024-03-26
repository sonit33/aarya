use crate::models::database::verification_code::VerificationCode;
use crate::tests::{setup_database, teardown_database};
use crate::utils::random::generate_guid;

#[tokio::test]
async fn test_create_or_update_student_code() {
	let db_name = generate_guid(8);
	let pool = setup_database(&db_name).await;

	// Create a student to satisfy foreign key constraint
	// let (student_id, _) = create_student_and_teacher(&pool).await;

	let student_id = 1;

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
	let teacher_id = 1;

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
