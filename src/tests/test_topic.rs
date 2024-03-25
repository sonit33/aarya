use sqlx::MySqlPool;

use crate::models::database::course::Course;
use crate::models::database::topic::Topic;
use crate::tests::{setup_database, teardown_database};
use crate::utils::random::generate_guid;

// Creates a course to satisfy the foreign key constraint for topics
async fn create_course_for_topic(pool: &MySqlPool) -> i32 {
	let course_result = Course::create(pool, "Course for Topic", Some("Course description for topic tests")).await.unwrap();
	let course_id = course_result.last_insert_id() as i32;
	course_id
}

#[tokio::test]
async fn test_create_topic() {
	let db_name = generate_guid(8);
	let pool = setup_database(&db_name).await;

	// Create a course to satisfy foreign key constraint in topic
	let course_id = create_course_for_topic(&pool).await;

	// Test creating a Topic
	let result = Topic::create(&pool, course_id, "Integration Testing with Rust", Some("Testing Topic creation")).await;
	assert!(result.is_ok());

	teardown_database(&pool, &db_name).await.unwrap();
}

#[tokio::test]
async fn test_read_topic() {
	let db_name = generate_guid(8);
	let pool = setup_database(&db_name).await;

	// Create a course and then a topic
	let course_id = create_course_for_topic(&pool).await;
	let create_result = Topic::create(&pool, course_id, "Reading Topic", Some("Testing Topic reading")).await.unwrap();
	let topic_id = create_result.last_insert_id() as i32;

	// Test reading the created topic
	let result = Topic::read(&pool, topic_id).await;
	assert!(result.is_ok());
	assert!(result.unwrap().is_some());

	teardown_database(&pool, &db_name).await.unwrap();
}

#[tokio::test]
async fn test_update_topic() {
	let db_name = generate_guid(8);
	let pool = setup_database(&db_name).await;

	// Create a course and then a topic
	let course_id = create_course_for_topic(&pool).await;
	let create_result = Topic::create(&pool, course_id, "Topic to Update", Some("Before update")).await.unwrap();
	let topic_id = create_result.last_insert_id() as i32;

	// Update the topic
	let update_result = Topic::update(&pool, topic_id, course_id, "Updated Topic Name", Some("After update")).await;
	assert!(update_result.is_ok());

	// Verify the update
	let read_result = Topic::read(&pool, topic_id).await.unwrap().unwrap();
	assert_eq!(read_result.topic_name, "Updated Topic Name");
	assert_eq!(read_result.description, Some("After update".to_string()));

	teardown_database(&pool, &db_name).await.unwrap();
}

#[tokio::test]
async fn test_delete_topic() {
	let db_name = generate_guid(8);
	let pool = setup_database(&db_name).await;

	// Create a course and then a topic
	let course_id = create_course_for_topic(&pool).await;
	let create_result = Topic::create(&pool, course_id, "Topic to Delete", Some("Testing deletion")).await.unwrap();
	let topic_id = create_result.last_insert_id() as i32;

	// Delete the topic
	let delete_result = Topic::delete(&pool, topic_id).await;
	assert!(delete_result.is_ok());

	// Attempt to read the deleted topic
	let read_result = Topic::read(&pool, topic_id).await;
	assert!(read_result.unwrap().is_none());

	teardown_database(&pool, &db_name).await.unwrap();
}
