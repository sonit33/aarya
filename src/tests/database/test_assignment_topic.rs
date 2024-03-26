use sqlx::MySqlPool;
use time::OffsetDateTime;

use crate::models::database::assignment::Assignment;
use crate::models::database::assignment_topic::AssignmentTopic;
use crate::models::database::course::Course;
use crate::models::database::teacher::Teacher;
use crate::models::database::topic::Topic;
use crate::tests::{setup_database, teardown_database};
use crate::utils::random::generate_guid;

// Function to create necessary referenced objects: Course, Teacher, Assignment, and Topic
async fn create_referenced_objects_for_assignment_topic(pool: &MySqlPool) -> (i32, i32) {
	// Create a course
	let course_result = Course::create(pool, "Course for AssignmentTopic", Some("Description")).await.unwrap();
	let course_id = course_result.last_insert_id() as i32;

	// Create a teacher
	let teacher_result = Teacher::create(
		pool,
		"Teacher for AssignmentTopic",
		Some("School"),
		"password",
		"teacher@example.com",
		Some("http://example.com/photo.jpg"),
		Some("Bio"),
		Some("Education"),
		Some("Skills"),
		Some("Certifications"),
		Some("Employed At"),
		1,
		1,
		Some("@venmoTeacher"),
		Some("teacher_paypal@example.com"),
		Some(1)
	).await.unwrap();
	let teacher_id = teacher_result.last_insert_id() as i32;

	// Create an assignment
	let assignment_result = Assignment::create(pool, course_id, teacher_id, "Assignment for AssignmentTopic", OffsetDateTime::now_utc()).await.unwrap();
	let assignment_id = assignment_result.last_insert_id() as i32;

	// Create a topic
	let topic_result = Topic::create(pool, course_id, "Topic for AssignmentTopic", Some("Topic Description")).await.unwrap();
	let topic_id = topic_result.last_insert_id() as i32;

	(assignment_id, topic_id)
}

#[tokio::test]
async fn test_create_assignment_topic() {
	let db_name = generate_guid(8);
	let pool = setup_database(&db_name).await;

	// Create referenced objects and get their IDs
	let (assignment_id, topic_id) = create_referenced_objects_for_assignment_topic(&pool).await;

	// Test creating an AssignmentTopic association
	let result = AssignmentTopic::create(&pool, assignment_id, topic_id).await;
	assert!(result.is_ok());

	teardown_database(&pool, &db_name).await.unwrap();
}

#[tokio::test]
async fn test_delete_assignment_topic() {
	let db_name = generate_guid(8);
	let pool = setup_database(&db_name).await;

	// Create referenced objects and get their IDs
	let (assignment_id, topic_id) = create_referenced_objects_for_assignment_topic(&pool).await;

	// Create an AssignmentTopic association and then delete it
	let _ = AssignmentTopic::create(&pool, assignment_id, topic_id).await.unwrap();
	let delete_result = AssignmentTopic::delete(&pool, assignment_id, topic_id).await;
	assert!(delete_result.is_ok());

	teardown_database(&pool, &db_name).await.unwrap();
}
