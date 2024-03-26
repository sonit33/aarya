use crate::models::database::teacher::Teacher;
use crate::tests::{setup_database, teardown_database};
use crate::utils::random::generate_guid;

#[tokio::test]
async fn test_create_teacher() {
	let db_name = generate_guid(8);
	let pool = setup_database(&db_name).await;

	let result = Teacher::create(
		&pool,
		"Alice Smith",
		Some("Springfield High"),
		"securepassword",
		"alice.smith@example.com",
		Some("http://example.com/photo.jpg"),
		Some("A brief bio here."),
		Some("Masters in Education"),
		Some("Math, Science"),
		Some("Certified Math Teacher"),
		Some("Springfield High"),
		1,
		1,
		Some("@alicevenmo"),
		Some("alicepay@example.com"),
		Some(1),
	).await;
	assert!(result.is_ok());

	teardown_database(&pool, &db_name).await.unwrap();
}

#[tokio::test]
async fn test_read_teacher() {
	let db_name = generate_guid(8);
	let pool = setup_database(&db_name).await;

	let x = Teacher::create(
		&pool,
		"Bob Johnson",
		Some("Rivertown Middle"),
		"anothersecurepassword",
		"bob.johnson@example.com",
		None,
		None,
		None,
		None,
		None,
		None,
		1,
		1,
		None,
		None,
		None,
	).await;
	let teacher_id = x.unwrap().last_insert_id() as i32; // Assuming this is the first entry and hence has ID 1
	let result = Teacher::read(&pool, teacher_id).await;
	assert!(result.is_ok());

	teardown_database(&pool, &db_name).await.unwrap();
}

#[tokio::test]
async fn test_update_teacher() {
	let db_name = generate_guid(8);
	let pool = setup_database(&db_name).await;

	let x = Teacher::create(
		&pool,
		"Charlie Kim",
		None,
		"password123",
		"charlie.kim@example.com",
		None,
		Some("A detailed blurb about teaching philosophy."),
		Some("PhD in Physics"),
		Some("Physics, Calculus"),
		Some("Physics Teacher Certification"),
		Some("Metro City College"),
		2,
		1,
		None,
		Some("charliepaypal@example.com"),
		Some(3),
	).await;
	let teacher_id = x.unwrap().last_insert_id() as i32; // Assuming this is the first entry and hence has ID 1
	let result = Teacher::update(
		&pool,
		teacher_id,
		"Charlie Kim",
		Some("Metro City College"),
		"newpassword123",
		"charlie.kim@example.com",
		Some("http://example.com/newphoto.jpg"),
		Some("Updated teaching philosophy blurb."),
		Some("PhD in Physics"),
		Some("Advanced Physics, Advanced Calculus"),
		Some("Advanced Physics Teacher Certification"),
		Some("Metro City College"),
		2,
		1,
		Some("@charlievenmo"),
		Some("charliepaypal@example.com"),
		Some(3),
	).await;
	assert!(result.is_ok());

	teardown_database(&pool, &db_name).await.unwrap();
}

#[tokio::test]
async fn test_delete_teacher() {
	let db_name = generate_guid(8);
	let pool = setup_database(&db_name).await;

	let x = Teacher::create(
		&pool,
		"Diana Lee",
		Some("Westside Elementary"),
		"securepassword456",
		"diana.lee@example.com",
		Some("http://example.com/diana.jpg"),
		Some("Teaching is a passion."),
		Some("Bachelor in Elementary Education"),
		Some("Literature, Writing"),
		Some("Teaching Certificate"),
		Some("Westside Elementary"),
		1,
		1,
		Some("@dianavenmo"),
		Some("dianapay@example.com"),
		Some(1),
	).await;
	let teacher_id = x.unwrap().last_insert_id() as i32; // Assuming this is the first entry and hence has ID 1
	let delete_result = Teacher::delete(&pool, teacher_id).await;
	assert!(delete_result.is_ok());

	teardown_database(&pool, &db_name).await.unwrap();
}
