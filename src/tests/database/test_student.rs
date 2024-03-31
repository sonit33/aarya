use time::OffsetDateTime;

use crate::models::database::student::Student;
use crate::tests::{ setup_database, teardown_database };
use crate::utils::random::generate_guid;

#[tokio::test]
async fn test_create_student() {
    let db_name = generate_guid(8);
    let pool = setup_database(&db_name).await;

    let result = Student::create(
        &pool,
        "John Doe",
        "john.doe@example.com",
        "password123",
        true,
        false,
        true
    ).await;
    assert!(result.is_ok());
    let result = result.unwrap();
    assert!(result.last_insert_id() > 0);

    teardown_database(&pool, &db_name).await.unwrap();
}

#[tokio::test]
async fn test_read_student() {
    let db_name = generate_guid(8);
    let pool = setup_database(&db_name).await;

    let created_res = Student::create(
        &pool,
        "John Doe",
        "john.doe@example.com",
        "password123",
        true,
        false,
        true
    ).await;
    let student_id = created_res.unwrap().last_insert_id() as i32; // Assuming this is the first entry and hence has ID 1
    let result = Student::read(&pool, student_id).await;
    assert!(result.is_ok());
    let student = result.unwrap();
    assert!(student.is_some());
    let student = student.unwrap();
    assert_eq!(student.student_id.unwrap(), student_id);

    teardown_database(&pool, &db_name).await.unwrap();
}

#[tokio::test]
async fn test_update_student() {
    let db_name = generate_guid(8);
    let pool = setup_database(&db_name).await;

    let created_res = Student::create(
        &pool,
        "John Doe",
        "john.doe@example.com",
        "password123",
        true,
        false,
        true
    ).await;
    let student_id = created_res.unwrap().last_insert_id() as i32;

    let student = Student {
        student_id: Some(student_id),
        first_name: "Jane Doe".to_string(),
        email_address: "jane.doe@example.com".to_string(),
        password: "newpassword123".to_string(),
        id_hash: "".to_string(),
        email_hash: "".to_string(),
        account_active: true,
        email_verified: true,
        over_13: true,
        added_timestamp: Some(OffsetDateTime::now_utc()),
        updated_timestamp: Some(OffsetDateTime::now_utc()),
        deleted_timestamp: Some(OffsetDateTime::now_utc()),
    };

    student.update(&pool).await.unwrap();

    let updated_student = Student::read(&pool, student_id).await.unwrap().unwrap();
    assert_eq!(updated_student.first_name, "Jane Doe");
    assert_eq!(updated_student.email_address, "jane.doe@example.com");

    teardown_database(&pool, &db_name).await.unwrap();
}

#[tokio::test]
async fn test_delete_student() {
    let db_name = generate_guid(8);
    let pool = setup_database(&db_name).await;

    let created_res = Student::create(
        &pool,
        "John Doe",
        "john.doe@example.com",
        "password123",
        true,
        false,
        true
    ).await;
    let student_id = created_res.unwrap().last_insert_id() as i32; // Assuming this is the first entry and hence has ID 1
    let delete_result = Student::delete(&pool, student_id).await;
    assert!(delete_result.is_ok());

    let read_result = Student::read(&pool, student_id).await;
    assert!(read_result.is_ok());
    assert!(read_result.unwrap().is_some()); // Checking if marked as deleted but not actually deleted

    teardown_database(&pool, &db_name).await.unwrap();
}
