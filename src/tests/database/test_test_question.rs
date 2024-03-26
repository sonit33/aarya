use sqlx::MySqlPool;

use crate::models::database::course::Course;
use crate::models::database::question::Question;
use crate::models::database::teacher::Teacher;
use crate::models::database::test::Test;
use crate::models::database::test_question::TestQuestion;
use crate::tests::{ setup_database, teardown_database };
use crate::utils::random::generate_guid;

async fn create_referenced_objects_for_test_question(pool: &MySqlPool) -> (i32, i32) {
    let course_result = Course::create(
        pool,
        "Course for TestQuestion",
        Some("Course description")
    ).await.unwrap();
    let course_id = course_result.last_insert_id() as i32;

    let teacher_result = Teacher::create(
        pool,
        "Teacher for TestQuestion",
        Some("School for TestQuestion"),
        "password123",
        "teacher_testquestion@example.com",
        Some("http://example.com/photo.jpg"),
        Some("Teacher blurb for TestQuestion"),
        Some("Teacher education for TestQuestion"),
        Some("Skills for TestQuestion"),
        Some("Certifications for TestQuestion"),
        Some("Employed at for TestQuestion"),
        1,
        1,
        Some("@teacher_venmo_testquestion"),
        Some("teacher_paypal_testquestion@example.com"),
        Some(1)
    ).await.unwrap();
    let teacher_id = teacher_result.last_insert_id() as i32;

    let test_result = Test::create(pool, course_id, "Test for TestQuestion", 1, 10).await.unwrap();
    let test_id = test_result.last_insert_id() as i32;

    let question_result = Question::create(
        pool,
        course_id,
        "Question for TestQuestion",
        "Answers for TestQuestion",
        Some(4),
        Some(2),
        Some("Difficulty reason"),
        Some("Answer explanation"),
        Some("Hint for question"),
        1
    ).await.unwrap();
    let question_id = question_result.last_insert_id() as i32;

    (test_id, question_id)
}

#[tokio::test]
async fn test_create_test_question() {
    let db_name = generate_guid(8);
    let pool = setup_database(&db_name).await;

    let (test_id, question_id) = create_referenced_objects_for_test_question(&pool).await;

    let result = TestQuestion::create(&pool, test_id, question_id).await;
    assert!(result.is_ok());

    teardown_database(&pool, &db_name).await.unwrap();
}

#[tokio::test]
async fn test_delete_test_question() {
    let db_name = generate_guid(8);
    let pool = setup_database(&db_name).await;

    let (test_id, question_id) = create_referenced_objects_for_test_question(&pool).await;

    let _ = TestQuestion::create(&pool, test_id, question_id).await.unwrap();

    let delete_result = TestQuestion::delete(&pool, test_id, question_id).await;
    assert!(delete_result.is_ok());

    teardown_database(&pool, &db_name).await.unwrap();
}
