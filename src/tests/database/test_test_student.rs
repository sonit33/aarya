use sqlx::MySqlPool;

use crate::models::database::course::Course;
use crate::models::database::student::Student;
use crate::models::database::teacher::Teacher;
use crate::models::database::test::Test;
use crate::models::database::test_student::TestStudent;
use crate::tests::{ setup_database, teardown_database };
use crate::utils::random::generate_guid;

// Function to create required referenced objects (Course, Teacher, Test, and Student) and return their IDs
async fn create_referenced_objects_for_test_student(pool: &MySqlPool) -> (i32, i32) {
    // Create a course
    let course_result = Course::create(
        pool,
        "Course for TestStudent",
        Some("Course description for TestStudent tests")
    ).await.unwrap();
    let course_id = course_result.last_insert_id() as i32;

    // Create a teacher
    let teacher_result = Teacher::create(
        pool,
        "Teacher for TestStudent",
        Some("School for TestStudent"),
        "password123",
        "teacher_teststudent@example.com",
        Some("http://example.com/photo.jpg"),
        Some("Teacher blurb for TestStudent"),
        Some("Teacher education for TestStudent"),
        Some("Skills for TestStudent"),
        Some("Certifications for TestStudent"),
        Some("Employed at for TestStudent"),
        1,
        1,
        Some("@teacher_venmo_teststudent"),
        Some("teacher_paypal_teststudent@example.com"),
        Some(1)
    ).await.unwrap();
    let teacher_id = teacher_result.last_insert_id() as i32;

    // Create a test
    let test_result = Test::create(pool, course_id, "Test for TestStudent", 1, 10).await.unwrap();
    let test_id = test_result.last_insert_id() as i32;

    // Create a student
    let student_result = Student::create(
        pool,
        "Student for TestStudent",
        "student_teststudent@example.com",
        "password",
        true,
        true,
        true
    ).await.unwrap();
    let student_id = student_result.last_insert_id() as i32;

    (test_id, student_id)
}

#[tokio::test]
async fn test_create_test_student() {
    let db_name = generate_guid(8);
    let pool = setup_database(&db_name).await;

    // Create necessary referenced objects
    let (test_id, student_id) = create_referenced_objects_for_test_student(&pool).await;

    // Test creating a TestStudent association
    let result = TestStudent::create(
        &pool,
        test_id,
        student_id,
        Some("AI feedback for TestStudent")
    ).await;
    assert!(result.is_ok());

    teardown_database(&pool, &db_name).await.unwrap();
}

#[tokio::test]
async fn test_delete_test_student() {
    let db_name = generate_guid(8);
    let pool = setup_database(&db_name).await;

    // Create necessary referenced objects
    let (test_id, student_id) = create_referenced_objects_for_test_student(&pool).await;

    // Create a TestStudent association and then delete it
    let _ = TestStudent::create(
        &pool,
        test_id,
        student_id,
        Some("AI feedback for deletion TestStudent")
    ).await.unwrap();
    let delete_result = TestStudent::delete(&pool, test_id, student_id).await;
    assert!(delete_result.is_ok());

    teardown_database(&pool, &db_name).await.unwrap();
}
