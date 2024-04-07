use crate::models::database::course::Course;
use crate::tests::scaffold;

#[tokio::test]
async fn test_create_course() {
    scaffold(|pool_arc| async move {
        let result = Course::create(&pool_arc, "Test Course", Some("This is a test course")).await;
        assert!(result.is_ok());
        let result = result.unwrap();
        assert!(result.last_insert_id() > 0);
    })
    .await;
}

// #[tokio::test]
// async fn test_read_course() {
//     let pool = setup_database().await;

//     let created = Course::create(&pool, "Test Course", Some("This is a test course")).await;
//     let course_id = created.unwrap().last_insert_id(); // Assuming this is the first entry and hence has ID 1
//     println!("test_read_course: course_id {}", course_id);
//     let result = Course::read(&pool, course_id as i32).await;
//     assert!(result.is_ok(), "Failed to read course: {:?}", result.err());
//     let course = result.unwrap();
//     assert!(course.is_some());
//     let course = course.unwrap();
//     assert_eq!(course.course_id, course_id as i32);

//     teardown_database(&pool, &db_name).await.unwrap();
// }

// #[tokio::test]
// async fn test_update_course() {
//     let pool = setup_database().await;

//     let _ = Course::create(&pool, "Test Course", Some("This is a test course")).await;
//     let course_id = 1; // Assuming this is the first entry and hence has ID 1
//     let result = Course::update(
//         &pool,
//         course_id,
//         "Updated Course",
//         Some("Updated description"),
//     )
//     .await;
//     assert!(result.is_ok());

//     let course = Course::read(&pool, course_id).await.unwrap().unwrap();
//     assert_eq!(course.name, "Updated Course");
//     assert_eq!(course.description.unwrap(), "Updated description");

//     teardown_database(&pool, &db_name).await.unwrap();
// }

// #[tokio::test]
// async fn test_delete_course() {
//     let db_name = generate_guid(8);
//     let pool = setup_database(&db_name).await;

//     let _ = Course::create(&pool, "Test Course", Some("This is a test course")).await;
//     let course_id = 1; // Assuming this is the first entry and hence has ID 1
//     let delete_result = Course::delete(&pool, course_id).await;
//     assert!(delete_result.is_ok());

//     let read_result = Course::read(&pool, course_id).await;
//     assert!(read_result.is_ok());
//     assert!(read_result.unwrap().is_none());

//     teardown_database(&pool, &db_name).await.unwrap();
// }
