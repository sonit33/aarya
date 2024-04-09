use aarya_models::database::course::Course;

// #[tokio::test]
// async fn test_create_course() {
//     scaffold(|pool_arc| async move {
//         let c1 = Course::new();
//         let r1 = c1.create(&pool_arc).await.unwrap();
//         assert!(r1.last_insert_id() > 0);
//     })
//     .await;
// }

// #[tokio::test]
// async fn test_update_course() {
//     scaffold(|pool_arc| async move {
//         // create a new course
//         let mut c1 = Course::new();
//         let r1 = c1.create(&pool_arc).await.unwrap();
//         // set the id
//         c1.course_id = Some(r1.last_insert_id() as u32);
//         // update the course
//         c1.name = String::from("random course");
//         c1.update(&pool_arc).await.unwrap();
//         // read
//         let r2 = c1.read(&pool_arc).await.unwrap().unwrap();
//         // assert
//         assert_eq!(r2.name, String::from("random course"));
//     })
//     .await;
// }

// #[tokio::test]
// async fn test_delete_course() {
//     scaffold(|pool_arc| async move {
//         // create a new course
//         let mut c1 = Course::new();
//         let r1 = c1.create(&pool_arc).await.unwrap();
//         // set the id
//         c1.course_id = Some(r1.last_insert_id() as u32);
//         // delete the course
//         c1.delete(&pool_arc).await.unwrap();
//         // read
//         let r2 = c1.read(&pool_arc).await.unwrap();
//         // assert
//         assert!(r2.is_none());
//     })
//     .await;
// }
