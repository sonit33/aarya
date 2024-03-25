use crate::models::database::feedback::Feedback;
use crate::tests::{setup_database, teardown_database};
use crate::utils::random::generate_guid;

#[tokio::test]
async fn test_create_feedback() {
	let db_name = generate_guid(8);
	let pool = setup_database(&db_name).await;

	let result = Feedback::create(&pool, Some(1), Some(1), Some(1), Some(1), Some(1)).await;
	assert!(result.is_ok());
	let result = result.unwrap();
	assert!(result.last_insert_id() > 0);

	teardown_database(&pool, &db_name).await;
}

#[tokio::test]
async fn test_read_feedback() {
	let db_name = generate_guid(8);
	let pool = setup_database(&db_name).await;

	let _ = Feedback::create(&pool, Some(1), Some(1), Some(1), Some(1), Some(1)).await;
	let feedback_id = 1; // Assuming this is the first entry and hence has ID 1
	let result = Feedback::read(&pool, feedback_id).await;
	assert!(result.is_ok());
	let feedback = result.unwrap();
	assert!(feedback.is_some());
	let feedback = feedback.unwrap();
	assert_eq!(feedback.feedback_id, feedback_id);

	teardown_database(&pool, &db_name).await;
}

#[tokio::test]
async fn test_update_feedback() {
	let db_name = generate_guid(8);
	let pool = setup_database(&db_name).await;

	let _ = Feedback::create(&pool, Some(1), Some(1), Some(1), Some(1), Some(1)).await;
	let feedback_id = 1; // Assuming this is the first entry and hence has ID 1
	let result = Feedback::update(&pool, feedback_id, Some(2), Some(2), Some(2), Some(2), Some(2)).await;
	assert!(result.is_ok(), "Failed to read feedback: {:?}", result.err());

	let feedback = Feedback::read(&pool, feedback_id).await.unwrap().unwrap();
	assert_eq!(feedback.course_id, Some(2));
	assert_eq!(feedback.test_id, Some(2));
	assert_eq!(feedback.question_id, Some(2));
	assert_eq!(feedback.teacher_id, Some(2));
	assert_eq!(feedback.student_id, Some(2));

	teardown_database(&pool, &db_name).await;
}

#[tokio::test]
async fn test_delete_feedback() {
	let db_name = generate_guid(8);
	let pool = setup_database(&db_name).await;

	let _ = Feedback::create(&pool, Some(1), Some(1), Some(1), Some(1), Some(1)).await;
	let feedback_id = 1; // Assuming this is the first entry and hence has ID 1
	let delete_result = Feedback::delete(&pool, feedback_id).await;
	assert!(delete_result.is_ok());

	let read_result = Feedback::read(&pool, feedback_id).await;
	assert!(read_result.is_ok());
	assert!(read_result.unwrap().is_none());

	teardown_database(&pool, &db_name).await;
}
