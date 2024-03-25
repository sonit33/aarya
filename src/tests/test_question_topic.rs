use sqlx::MySqlPool;

use crate::models::database::course::Course;
use crate::models::database::question::Question;
use crate::models::database::question_topic::QuestionTopic;
use crate::models::database::topic::Topic;
use crate::tests::{setup_database, teardown_database};
use crate::utils::random::generate_guid;

// Function to create necessary objects: Course, Question, and Topic, to satisfy foreign key constraints
async fn create_referenced_objects_for_question_topic(pool: &MySqlPool) -> (i32, i32) {
	// Create a course to satisfy the foreign key constraint of the question and topic
	let course_result = Course::create(pool, "Course for QuestionTopic", Some("Description for course in QuestionTopic tests")).await.unwrap();
	let course_id = course_result.last_insert_id() as i32;

	// Create a question associated with the course
	let question_result = Question::create(pool, course_id, "What is Rust?", "A programming language", Some(4), Some(1), Some("Difficulty reason"), Some("Explanation"), Some("Hint"), 1).await.unwrap();
	let question_id = question_result.last_insert_id() as i32;

	// Create a topic associated with the course
	let topic_result = Topic::create(pool, course_id, "Rust Programming", Some("A topic on Rust programming")).await.unwrap();
	let topic_id = topic_result.last_insert_id() as i32;

	(question_id, topic_id)
}

#[tokio::test]
async fn test_create_question_topic() {
	let db_name = generate_guid(8);
	let pool = setup_database(&db_name).await;

	// Create referenced objects and get their IDs
	let (question_id, topic_id) = create_referenced_objects_for_question_topic(&pool).await;

	// Test creating a QuestionTopic association
	let result = QuestionTopic::create(&pool, question_id, topic_id).await;
	assert!(result.is_ok());

	teardown_database(&pool, &db_name).await.unwrap();
}

#[tokio::test]
async fn test_delete_question_topic() {
	let db_name = generate_guid(8);
	let pool = setup_database(&db_name).await;

	// Create referenced objects and get their IDs
	let (question_id, topic_id) = create_referenced_objects_for_question_topic(&pool).await;

	// Create a QuestionTopic association and then delete it
	let _ = QuestionTopic::create(&pool, question_id, topic_id).await.unwrap();
	let delete_result = QuestionTopic::delete(&pool, question_id, topic_id).await;
	assert!(delete_result.is_ok());

	teardown_database(&pool, &db_name).await.unwrap();
}
