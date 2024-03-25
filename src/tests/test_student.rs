use crate::models::database::student::Student;
use crate::tests::{setup_database, teardown_database};
use crate::utils::random::generate_guid;

#[tokio::test]
async fn test_create_student() {
	let db_name = generate_guid(8);
	let pool = setup_database(&db_name).await;

	let result = Student::create(&pool, "John Doe", "john.doe@example.com", "password123", 1, 0, 1).await;
	assert!(result.is_ok());
	let result = result.unwrap();
	assert!(result.last_insert_id() > 0);

	teardown_database(&pool, &db_name).await.unwrap();
}

#[tokio::test]
async fn test_read_student() {
	let db_name = generate_guid(8);
	let pool = setup_database(&db_name).await;

	let created_res = Student::create(&pool, "John Doe", "john.doe@example.com", "password123", 1, 0, 1).await;
	let student_id = created_res.unwrap().last_insert_id() as i32; // Assuming this is the first entry and hence has ID 1
	let result = Student::read(&pool, student_id).await;
	assert!(result.is_ok());
	let student = result.unwrap();
	assert!(student.is_some());
	let student = student.unwrap();
	assert_eq!(student.student_id, student_id);

	teardown_database(&pool, &db_name).await.unwrap();
}

#[tokio::test]
async fn test_update_student() {
	let db_name = generate_guid(8);
	let pool = setup_database(&db_name).await;

	let created_res = Student::create(&pool, "John Doe", "john.doe@example.com", "password123", 1, 0, 1).await;
	let student_id = created_res.unwrap().last_insert_id() as i32;
	; // Assuming this is the first entry and hence has ID 1
	let result = Student::update(&pool, student_id, "Jane Doe", "jane.doe@example.com", "newpassword123", 1, 1, 1).await;
	assert!(result.is_ok());

	let student = Student::read(&pool, student_id).await.unwrap().unwrap();
	assert_eq!(student.first_name, "Jane Doe");
	assert_eq!(student.email_address, "jane.doe@example.com");

	teardown_database(&pool, &db_name).await.unwrap();
}

#[tokio::test]
async fn test_delete_student() {
	let db_name = generate_guid(8);
	let pool = setup_database(&db_name).await;

	let created_res = Student::create(&pool, "John Doe", "john.doe@example.com", "password123", 1, 0, 1).await;
	let student_id = created_res.unwrap().last_insert_id() as i32;
	; // Assuming this is the first entry and hence has ID 1
	let delete_result = Student::delete(&pool, student_id).await;
	assert!(delete_result.is_ok());

	let read_result = Student::read(&pool, student_id).await;
	assert!(read_result.is_ok());
	assert!(read_result.unwrap().is_some()); // Checking if marked as deleted but not actually deleted

	teardown_database(&pool, &db_name).await.unwrap();
}
