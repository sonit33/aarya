use sqlx::MySqlPool;

use crate::models::database::payment::Payment;
use crate::models::database::student::Student;
use crate::models::database::teacher::Teacher;
use crate::tests::{setup_database, teardown_database};
use crate::utils::random::generate_guid;

async fn create_referenced_objects_for_payment(pool: &MySqlPool) -> (i32, i32) {
	let student_result = Student::create(pool, "Student Name", "student@example.com", "password", 1, 1, 1).await.unwrap();
	let student_id = student_result.last_insert_id() as i32;

	let teacher_result = Teacher::create(
		pool,
		"Teacher Name",
		Some("Teacher School"),
		"password",
		"teacher@example.com",
		Some("http://example.com/photo.jpg"),
		Some("A brief blurb"),
		Some("Teacher's education"),
		Some("Skills"),
		Some("Certifications"),
		Some("Employed at"),
		1,
		1,
		Some("@teacher_venmo"),
		Some("teacher_paypal@example.com"),
		Some(1)
	).await.unwrap();
	let teacher_id = teacher_result.last_insert_id() as i32;

	(student_id, teacher_id)
}

#[tokio::test]
async fn test_create_payment() {
	let db_name = generate_guid(8);
	let pool = setup_database(&db_name).await;

	let (student_id, teacher_id) = create_referenced_objects_for_payment(&pool).await;

	let result = Payment::create(
		&pool,
		1,
		100.00,
		None,
		Some(student_id),
		Some(teacher_id),
		1
	).await;
	assert!(result.is_ok());

	teardown_database(&pool, &db_name).await.unwrap();
}

#[tokio::test]
async fn test_read_payment() {
	let db_name = generate_guid(8);
	let pool = setup_database(&db_name).await;

	let (student_id, teacher_id) = create_referenced_objects_for_payment(&pool).await;

	let payment_result = Payment::create(
		&pool,
		1,
		200.00,
		None,
		Some(student_id),
		Some(teacher_id),
		1
	).await.unwrap();
	let payment_id = payment_result.last_insert_id() as i32;

	let result = Payment::read(&pool, payment_id).await;
	assert!(result.is_ok(), "Failed to read result: {:?}", result.err());

	teardown_database(&pool, &db_name).await.unwrap();
}

#[tokio::test]
async fn test_update_payment() {
	let db_name = generate_guid(8);
	let pool = setup_database(&db_name).await;

	let (student_id, teacher_id) = create_referenced_objects_for_payment(&pool).await;

	let payment_result = Payment::create(
		&pool,
		1,
		300.00,
		None,
		Some(student_id),
		Some(teacher_id),
		1
	).await.unwrap();
	let payment_id = payment_result.last_insert_id() as i32;

	let update_result = Payment::update(
		&pool,
		payment_id,
		2,
		150.00,
		Some(payment_id), // Assuming p_id is the same as payment_id for this test
		Some(student_id),
		Some(teacher_id),
		2
	).await;
	assert!(update_result.is_ok());

	teardown_database(&pool, &db_name).await.unwrap();
}

#[tokio::test]
async fn test_delete_payment() {
	let db_name = generate_guid(8);
	let pool = setup_database(&db_name).await;

	let (student_id, teacher_id) = create_referenced_objects_for_payment(&pool).await;

	let payment_result = Payment::create(
		&pool,
		1,
		400.00,
		None,
		Some(student_id),
		Some(teacher_id),
		1
	).await.unwrap();
	let payment_id = payment_result.last_insert_id() as i32;

	let delete_result = Payment::delete(&pool, payment_id).await;
	assert!(delete_result.is_ok());

	teardown_database(&pool, &db_name).await.unwrap();
}
