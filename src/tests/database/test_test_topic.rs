use sqlx::MySqlPool;

use crate::models::database::course::Course;
use crate::models::database::test::Test;
use crate::models::database::test_topic::TestTopic;
use crate::models::database::topic::Topic;
use crate::tests::{setup_database, teardown_database};
use crate::utils::random::generate_guid;

// Function to create necessary objects: Course, Test, and Topic, to satisfy foreign key constraints for TestTopic
async fn create_referenced_objects_for_test_topic(pool: &MySqlPool) -> (i32, i32) {
	// Create a course to be associated with the test and topic
	let course_result = Course::create(pool, "Course for TestTopic", Some("Description for course in TestTopic tests")).await.unwrap();
	let course_id = course_result.last_insert_id() as i32;

	// Create a test associated with the course
	let test_result = Test::create(pool, course_id, "Test for TestTopic", 1, 10).await.unwrap();
	let test_id = test_result.last_insert_id() as i32;

	// Create a topic associated with the course
	let topic_result = Topic::create(pool, course_id, "Topic for TestTopic", Some("Description for topic in TestTopic tests")).await.unwrap();
	let topic_id = topic_result.last_insert_id() as i32;

	(test_id, topic_id)
}

#[tokio::test]
async fn test_create_test_topic() {
	let db_name = generate_guid(8);
	let pool = setup_database(&db_name).await;

	// Create referenced objects and get their IDs
	let (test_id, topic_id) = create_referenced_objects_for_test_topic(&pool).await;

	// Test creating a TestTopic association
	let result = TestTopic::create(&pool, test_id, topic_id).await;
	assert!(result.is_ok());

	teardown_database(&pool, &db_name).await.unwrap();
}

#[tokio::test]
async fn test_delete_test_topic() {
	let db_name = generate_guid(8);
	let pool = setup_database(&db_name).await;

	// Create referenced objects and get their IDs
	let (test_id, topic_id) = create_referenced_objects_for_test_topic(&pool).await;

	// Create a TestTopic association and then delete it
	let _ = TestTopic::create(&pool, test_id, topic_id).await.unwrap();
	let delete_result = TestTopic::delete(&pool, test_id, topic_id).await;
	assert!(delete_result.is_ok());

	teardown_database(&pool, &db_name).await.unwrap();
}
