use crate::models::database::course::Course;
use crate::models::database::question::Question;
use crate::tests::{setup_database, teardown_database};
use crate::utils::random::generate_guid;

#[tokio::test]
async fn test_create_question() {
	let db_name = generate_guid(8);
	let pool = setup_database(&db_name).await;

	// First, create a course to satisfy the foreign key constraint
	let course_result = Course::create(&pool, "Sample Course", Some("Sample Description")).await;
	assert!(course_result.is_ok());
	let course = course_result.unwrap();
	let course_id = course.last_insert_id(); // Adjust this line based on how you retrieve IDs

	// Then, create a question associated with the newly created course
	let result = Question::create(&pool, course_id as i32, "What is Rust?", "A programming language.", Some(4), Some(3), Some("Because it's important."), Some("It's widely used for system programming."), Some("Think about system programming."), 1).await;
	assert!(result.is_ok());
	let result = result.unwrap();
	assert!(result.last_insert_id() > 0);

	teardown_database(&pool, &db_name).await.unwrap();
}


#[tokio::test]
async fn test_read_question() {
	let db_name = generate_guid(8);
	let pool = setup_database(&db_name).await;

	let course_result = Course::create(&pool, "Sample Course", Some("Sample Description")).await;
	assert!(course_result.is_ok());
	let course = course_result.unwrap();
	let course_id = course.last_insert_id() as i32; // Adjust this line based on how you retrieve IDs

	let _ = Question::create(&pool, course_id, "What is Rust?", "A programming language.", Some(4), Some(3), Some("Because it's important."), Some("It's widely used for system programming."), Some("Think about system programming."), 1).await;
	let question_id = 1; // Assuming this is the first entry and hence has ID 1
	let result = Question::read(&pool, question_id).await;
	assert!(result.is_ok());
	let question = result.unwrap();
	assert!(question.is_some());
	let question = question.unwrap();
	assert_eq!(question.question_id, question_id);

	teardown_database(&pool, &db_name).await.unwrap();
}

#[tokio::test]
async fn test_update_question() {
	let db_name = generate_guid(8);
	let pool = setup_database(&db_name).await;

	let course_result = Course::create(&pool, "Sample Course", Some("Sample Description")).await;
	assert!(course_result.is_ok());
	let course = course_result.unwrap();
	let course_id = course.last_insert_id() as i32; // Adjust this line based on how you retrieve IDs

	let created_response = Question::create(&pool, course_id, "What is Rust?", "A programming language.", Some(4), Some(3), Some("Because it's important."), Some("It's widely used for system programming."), Some("Think about system programming."), 1).await;
	let question_id = created_response.unwrap().last_insert_id() as i32; // Assuming this is the first entry and hence has ID 1
	let result = Question::update(&pool, question_id, 1, "Updated Question?", "Updated Answer", None, None, Some("Updated reason"), Some("Updated explanation"), Some("Updated hint"), 2).await;
	assert!(result.is_ok());

	let question = Question::read(&pool, question_id).await.unwrap().unwrap();
	assert_eq!(question.question, "Updated Question?");
	assert_eq!(question.answers, "Updated Answer");

	teardown_database(&pool, &db_name).await.unwrap();
}

#[tokio::test]
async fn test_delete_question() {
	let db_name = generate_guid(8);
	let pool = setup_database(&db_name).await;

	let _ = Question::create(&pool, 1, "What is Rust?", "A programming language.", Some(4), Some(3), Some("Because it's important."), Some("It's widely used for system programming."), Some("Think about system programming."), 1).await;
	let question_id = 1; // Assuming this is the first entry and hence has ID 1
	let delete_result = Question::delete(&pool, question_id).await;
	assert!(delete_result.is_ok());

	let read_result = Question::read(&pool, question_id).await;
	assert!(read_result.is_ok());
	assert!(read_result.unwrap().is_none());

	teardown_database(&pool, &db_name).await.unwrap();
}
