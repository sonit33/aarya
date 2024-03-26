use sqlx::MySqlPool;

use crate::models::database::course::Course;
use crate::models::database::test::Test;
use crate::tests::{setup_database, teardown_database};
use crate::utils::random::generate_guid;

async fn create_course_for_test(pool: &MySqlPool) -> i32 {
	let course_result = Course::create(pool, "Integration Test Course", Some("Description for the course used in integration tests.")).await.unwrap();
	let course_id = course_result.last_insert_id() as i32;
	course_id
}

#[tokio::test]
async fn test_create_test() {
	let db_name = generate_guid(8);
	let pool = setup_database(&db_name).await;

	let course_id = create_course_for_test(&pool).await;

	let result = Test::create(&pool, course_id, "Test Name", 1, 10).await;
	assert!(result.is_ok());

	teardown_database(&pool, &db_name).await.unwrap();
}

#[tokio::test]
async fn test_read_test() {
	let db_name = generate_guid(8);
	let pool = setup_database(&db_name).await;

	let course_id = create_course_for_test(&pool).await;

	let create_result = Test::create(&pool, course_id, "Test to Read", 2, 15).await.unwrap();
	let test_id = create_result.last_insert_id() as i32;

	let result = Test::read(&pool, test_id).await;
	assert!(result.is_ok());
	assert!(result.unwrap().is_some());

	teardown_database(&pool, &db_name).await.unwrap();
}

#[tokio::test]
async fn test_update_test() {
	let db_name = generate_guid(8);
	let pool = setup_database(&db_name).await;

	let course_id = create_course_for_test(&pool).await;

	let create_result = Test::create(&pool, course_id, "Test to Update", 1, 20).await.unwrap();
	let test_id = create_result.last_insert_id() as i32;

	let update_result = Test::update(&pool, test_id, course_id, "Updated Test Name", 3, 25).await;
	assert!(update_result.is_ok());

	let read_result = Test::read(&pool, test_id).await.unwrap().unwrap();
	assert_eq!(read_result.test_name, "Updated Test Name");
	assert_eq!(read_result.mode, 3);
	assert_eq!(read_result.test_size, 25);

	teardown_database(&pool, &db_name).await.unwrap();
}

#[tokio::test]
async fn test_delete_test() {
	let db_name = generate_guid(8);
	let pool = setup_database(&db_name).await;

	let course_id = create_course_for_test(&pool).await;

	let create_result = Test::create(&pool, course_id, "Test to Delete", 2, 30).await.unwrap();
	let test_id = create_result.last_insert_id() as i32;

	let delete_result = Test::delete(&pool, test_id).await;
	assert!(delete_result.is_ok());

	let read_result = Test::read(&pool, test_id).await;
	assert!(read_result.unwrap().is_none());

	teardown_database(&pool, &db_name).await.unwrap();
}
