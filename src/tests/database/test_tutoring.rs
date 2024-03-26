use sqlx::MySqlPool;

use crate::models::database::course::Course;
use crate::models::database::student::Student;
use crate::models::database::teacher::Teacher;
use crate::models::database::tutoring::Tutoring;
use crate::tests::{ setup_database, teardown_database };
use crate::utils::random::generate_guid;

// Creates necessary objects for Tutoring: Course, Student, and Teacher
async fn create_objects_for_tutoring(pool: &MySqlPool) -> (i32, i32, i32) {
    // Create a course
    let course_result = Course::create(
        pool,
        "Course for Tutoring",
        Some("This is a course for tutoring sessions.")
    ).await.unwrap();
    let course_id = course_result.last_insert_id() as i32;

    // Create a student
    let student_result = Student::create(
        pool,
        "Student for Tutoring",
        "student.tutoring@example.com",
        "password123",
        true,
        true,
        true
    ).await.unwrap();
    let student_id = student_result.last_insert_id() as i32;

    // Create a teacher
    let teacher_result = Teacher::create(
        pool,
        "Teacher for Tutoring",
        Some("School for Tutoring"),
        "securepassword",
        "teacher.tutoring@example.com",
        Some("http://example.com/teacher.jpg"),
        Some("Teaching is my passion."),
        Some("Masters in Teaching"),
        Some("Math, Science"),
        Some("Certified Teacher"),
        Some("Local High School"),
        1,
        1,
        Some("@teacherVenmo"),
        Some("teacherPaypal@example.com"),
        Some(1)
    ).await.unwrap();
    let teacher_id = teacher_result.last_insert_id() as i32;

    (course_id, student_id, teacher_id)
}

#[tokio::test]
async fn test_create_tutoring_session() {
    let db_name = generate_guid(8);
    let pool = setup_database(&db_name).await;

    let (course_id, student_id, teacher_id) = create_objects_for_tutoring(&pool).await;

    // Test creating a tutoring session
    let result = Tutoring::create(&pool, course_id, student_id, teacher_id, 1).await;
    assert!(result.is_ok());

    teardown_database(&pool, &db_name).await.unwrap();
}

#[tokio::test]
async fn test_delete_tutoring_session() {
    let db_name = generate_guid(8);
    let pool = setup_database(&db_name).await;

    let (course_id, student_id, teacher_id) = create_objects_for_tutoring(&pool).await;

    let create_result = Tutoring::create(
        &pool,
        course_id,
        student_id,
        teacher_id,
        2
    ).await.unwrap();
    let tutoring_id = create_result.last_insert_id() as i32;

    // Test deleting the tutoring session
    let delete_result = Tutoring::delete(&pool, tutoring_id).await;
    assert!(delete_result.is_ok());

    teardown_database(&pool, &db_name).await.unwrap();
}

#[tokio::test]
async fn test_update_tutoring_session() {
    let db_name = generate_guid(8);
    let pool = setup_database(&db_name).await;

    // Create necessary objects and get their IDs
    let (course_id, student_id, teacher_id) = create_objects_for_tutoring(&pool).await;

    // Create a tutoring session
    let create_result = Tutoring::create(
        &pool,
        course_id,
        student_id,
        teacher_id,
        1
    ).await.unwrap();
    let tutoring_id = create_result.last_insert_id() as i32;

    // Update the tutoring session
    let update_result = Tutoring::update(
        &pool,
        tutoring_id,
        course_id,
        student_id,
        teacher_id,
        2
    ).await;
    assert!(update_result.is_ok());

    // Verify the update
    let read_result = Tutoring::read(&pool, tutoring_id).await.unwrap().unwrap();
    assert_eq!(read_result.t_type, 2);

    teardown_database(&pool, &db_name).await.unwrap();
}

#[tokio::test]
async fn test_read_tutoring_session() {
    let db_name = generate_guid(8);
    let pool = setup_database(&db_name).await;

    // Create necessary objects and get their IDs
    let (course_id, student_id, teacher_id) = create_objects_for_tutoring(&pool).await;

    // Create a tutoring session
    let create_result = Tutoring::create(
        &pool,
        course_id,
        student_id,
        teacher_id,
        1
    ).await.unwrap();
    let tutoring_id = create_result.last_insert_id() as i32;

    // Read the created tutoring session
    let read_result = Tutoring::read(&pool, tutoring_id).await;
    assert!(read_result.is_ok());
    let tutoring_session = read_result.unwrap().unwrap();
    assert_eq!(tutoring_session.course_id, course_id);
    assert_eq!(tutoring_session.student_id, student_id);
    assert_eq!(tutoring_session.teacher_id, teacher_id);
    assert_eq!(tutoring_session.t_type, 1);

    teardown_database(&pool, &db_name).await.unwrap();
}
