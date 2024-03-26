use sqlx::MySqlPool;
use time::OffsetDateTime;

use crate::models::database::assignment::Assignment;
use crate::models::database::assignment_student::AssignmentStudent;
use crate::models::database::course::Course;
use crate::models::database::student::Student;
use crate::models::database::teacher::Teacher;
use crate::tests::{ setup_database, teardown_database };
use crate::utils::random::generate_guid;

async fn create_referenced_objects_for_assignment_student(pool: &MySqlPool) -> (i32, i32) {
    let db_name = generate_guid(8);
    let course_result = Course::create(
        pool,
        "Test Course",
        Some("This is a test course")
    ).await.unwrap();
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

    let assignment_result = Assignment::create(
        pool,
        course_id,
        teacher_id,
        "Test Assignment",
        OffsetDateTime::now_utc()
    ).await.unwrap();
    let assignment_id = assignment_result.last_insert_id() as i32;

    let student_result = Student::create(
        pool,
        "Student Name",
        "student@example.com",
        "password",
        true,
        true,
        true
    ).await.unwrap();
    let student_id = student_result.last_insert_id() as i32;

    (assignment_id, student_id)
}

#[tokio::test]
async fn test_create_assignment_student() {
    let db_name = generate_guid(8);
    let pool = setup_database(&db_name).await;

    let (assignment_id, student_id) = create_referenced_objects_for_assignment_student(&pool).await;

    let result = AssignmentStudent::create(&pool, assignment_id, student_id).await;
    assert!(result.is_ok());

    teardown_database(&pool, &db_name).await.unwrap();
}

#[tokio::test]
async fn test_delete_assignment_student() {
    let db_name = generate_guid(8);
    let pool = setup_database(&db_name).await;

    let (assignment_id, student_id) = create_referenced_objects_for_assignment_student(&pool).await;

    let _ = AssignmentStudent::create(&pool, assignment_id, student_id).await.unwrap();

    let delete_result = AssignmentStudent::delete(&pool, assignment_id, student_id).await;
    assert!(delete_result.is_ok());

    teardown_database(&pool, &db_name).await.unwrap();
}
