use crate::models::database::course::Course;
use crate::models::database::question::Question;
use crate::tests::{setup_database, teardown_database};
use crate::utils::random::generate_guid;

#[tokio::test]
async fn test_create_question() {
    let db_name = generate_guid(8);
    let pool = setup_database(&db_name).await;

    // First, create a course to satisfy the foreign key constraint
    let course_result = Course::new().create(&pool).await;
    assert!(course_result.is_ok());
    let course = course_result.unwrap();

    let mut question = Question::new();
    question.course_id = course.last_insert_id() as u32;

    // Then, create a question associated with the newly created course
    let result = question.create(&pool).await;
    assert!(result.is_ok());
    let result = result.unwrap();
    assert!(result.last_insert_id() > 0);

    teardown_database(&pool, &db_name).await.unwrap();
}

#[tokio::test]
async fn test_read_question() {
    let db_name = generate_guid(8);
    let pool = setup_database(&db_name).await;

    let course_result = Course::new().create(&pool).await;
    assert!(course_result.is_ok());
    let course = course_result.unwrap();
    let mut q1 = Question::new();
    q1.course_id = course.last_insert_id() as u32;

    // Then, create a question associated with the newly created course
    let result = q1.create(&pool).await.unwrap();
    q1.question_id = Some(result.last_insert_id() as u32);
    let result = q1.read(&pool).await;
    assert!(result.is_ok());
    let q2 = result.unwrap();
    assert!(q2.is_some());
    assert_eq!(q2.unwrap().question_id.unwrap(), q1.question_id.unwrap());

    teardown_database(&pool, &db_name).await.unwrap();
}

#[tokio::test]
async fn test_update_question() {
    let db_name = generate_guid(8);
    let pool = setup_database(&db_name).await;

    let course_result = Course::new().create(&pool).await;
    assert!(course_result.is_ok());
    let course = course_result.unwrap();

    // create a new question
    let mut q1 = Question::new();
    q1.course_id = course.last_insert_id() as u32;
    let r1 = q1.create(&pool).await;

    // update the question
    // let mut q2 = Question::new();
    q1.question_id = Some(r1.unwrap().last_insert_id() as u32); // Assuming this is the first entry and hence has ID 1
    q1.q_text = String::from("Updated question?");
    let r2 = q1.update(&pool).await.unwrap();
    assert!(r2.rows_affected() > 0);

    // read the updated question to verify change
    let q3 = q1.read(&pool).await.unwrap().unwrap();
    assert_eq!(q3.q_text, q1.q_text);

    teardown_database(&pool, &db_name).await.unwrap();
}

#[tokio::test]
async fn test_delete_question() {
    let db_name = generate_guid(8);
    let pool = setup_database(&db_name).await;

    // create a new question
    let mut q1 = Question::new();
    q1.course_id = 1;
    q1.chapter_id = 1;
    let r1 = q1.create(&pool).await.unwrap();
    q1.question_id = Some(r1.last_insert_id() as u32);
    // delete the question
    q1.delete(&pool).await.unwrap();
    // verify delete
    let r2 = q1.read(&pool).await.unwrap();
    assert!(r2.is_none());

    teardown_database(&pool, &db_name).await.unwrap();
}
