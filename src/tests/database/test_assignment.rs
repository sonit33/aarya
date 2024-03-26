use sqlx::MySqlPool;
use time::OffsetDateTime;

use crate::models::database::assignment::Assignment;
use crate::models::database::course::Course;
use crate::models::database::teacher::Teacher;
use crate::tests::{setup_database, teardown_database};
use crate::utils::random::generate_guid;

async fn create_referenced_objects(pool: &MySqlPool) -> (i32, i32) {
	let course_result = Course::create(pool, "Test Course", Some("This is a test course")).await.unwrap();
	let course_id = course_result.last_insert_id() as i32;

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

	(course_id, teacher_id)
}

#[tokio::test]
async fn test_create_assignment() {
	let db_name = generate_guid(8);
	let pool = setup_database(&db_name).await;

	let (course_id, teacher_id) = create_referenced_objects(&pool).await;
	let due_on_timestamp = OffsetDateTime::now_utc();

	let result = Assignment::create(
		&pool,
		course_id,
		teacher_id,
		"Test Assignment",
		due_on_timestamp
	).await;
	assert!(result.is_ok());

	teardown_database(&pool, &db_name).await.unwrap();
}

#[tokio::test]
async fn test_read_assignment() {
	let db_name = generate_guid(8);
	let pool = setup_database(&db_name).await;

	let (course_id, teacher_id) = create_referenced_objects(&pool).await;
	let due_on_timestamp = OffsetDateTime::now_utc();

	let assignment_result = Assignment::create(
		&pool,
		course_id,
		teacher_id,
		"Test Assignment",
		due_on_timestamp
	).await.unwrap();
	let assignment_id = assignment_result.last_insert_id() as i32;

	let result = Assignment::read(&pool, assignment_id).await;
	assert!(result.is_ok());

	teardown_database(&pool, &db_name).await.unwrap();
}

#[tokio::test]
async fn test_update_assignment() {
	let db_name = generate_guid(8);
	let pool = setup_database(&db_name).await;

	let (course_id, teacher_id) = create_referenced_objects(&pool).await;
	let due_on_timestamp = OffsetDateTime::now_utc();

	let assignment_result = Assignment::create(
		&pool,
		course_id,
		teacher_id,
		"Initial Assignment",
		due_on_timestamp
	).await.unwrap();
	let assignment_id = assignment_result.last_insert_id() as i32;

	let update_result = Assignment::update(
		&pool,
		assignment_id,
		course_id,
		teacher_id,
		"Updated Assignment",
		due_on_timestamp
	).await;
	assert!(update_result.is_ok());

	teardown_database(&pool, &db_name).await.unwrap();
}

#[tokio::test]
async fn test_delete_assignment() {
	let db_name = generate_guid(8);
	let pool = setup_database(&db_name).await;

	let (course_id, teacher_id) = create_referenced_objects(&pool).await;
	let due_on_timestamp = OffsetDateTime::now_utc();

	let assignment_result = Assignment::create(
		&pool,
		course_id,
		teacher_id,
		"Assignment to Delete",
		due_on_timestamp
	).await.unwrap();
	let assignment_id = assignment_result.last_insert_id() as i32;

	let delete_result = Assignment::delete(&pool, assignment_id).await;
	assert!(delete_result.is_ok());

	teardown_database(&pool, &db_name).await.unwrap();
}
