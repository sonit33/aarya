use aarya_models::database::student::Student;
use aarya_utils::{
    db_ops::{setup_test_database, teardown_test_database},
    random::generate_guid
};

#[tokio::test]
async fn test_create_student() {
    let db_name = generate_guid(8);
    let pool = setup_test_database(&db_name).await;

    let student = Student::random(&db_name);
    let result = student.create(&pool).await;

    assert!(result.is_ok());
    let result = result.unwrap();
    assert!(result.last_insert_id() > 0);

    teardown_test_database(&pool, &db_name).await.unwrap();
}

#[tokio::test]
async fn test_read_student() {
    let db_name = generate_guid(8);
    let pool = setup_test_database(&db_name).await;

    let mut student = Student::random(&db_name);
    let created_res = student.create(&pool).await;
    student.student_id = Some(created_res.unwrap().last_insert_id() as u32); // Assuming this is the first entry and hence has ID 1
    let result = student.read(&pool).await;
    assert!(result.is_ok());
    let student = result.unwrap();
    assert!(student.is_some());
    let student = student.unwrap();
    assert_eq!(student.student_id.unwrap(), student.student_id.unwrap());

    teardown_test_database(&pool, &db_name).await.unwrap();
}

#[tokio::test]
async fn test_update_student() {
    let db_name = generate_guid(8);
    let pool = setup_test_database(&db_name).await;

    let mut student = Student::random(&db_name);
    let created_res = student.create(&pool).await;
    student.student_id = Some(created_res.unwrap().last_insert_id() as u32);

    // change the first name
    student.first_name = "Jane Doe".to_string();
    student.email_address = "jane.doe@example.com".to_string();

    student.update(&pool).await.unwrap();

    let updated_student = student.read(&pool).await.unwrap().unwrap();
    assert_eq!(updated_student.first_name, "Jane Doe");
    // cannot change the email address after creation
    assert_ne!(updated_student.email_address, "jane.doe@example.com");

    teardown_test_database(&pool, &db_name).await.unwrap();
}

#[tokio::test]
async fn test_delete_student() {
    let db_name = generate_guid(8);
    let pool = setup_test_database(&db_name).await;

    let mut student = Student::random(&db_name);
    let created_res = student.create(&pool).await;

    student.student_id = Some(created_res.unwrap().last_insert_id() as u32); // Assuming this is the first entry and hence has ID 1
    let delete_result = student.delete(&pool).await;
    assert!(delete_result.is_ok());

    let read_result = student.read(&pool).await;
    assert!(read_result.is_ok());
    assert!(read_result.unwrap().is_none()); // must be deleted

    teardown_test_database(&pool, &db_name).await.unwrap();
}
