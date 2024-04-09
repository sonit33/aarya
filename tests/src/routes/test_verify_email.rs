use actix_web::{http, test, web, App};
use sqlx::{MySql, Pool};

use crate::{
    models::{
        auth::verify_email::VerifyEmailModel,
        database::{student::Student, verification_code::VerificationCode},
    },
    routes::auth::verify_email::verify_email_post,
    tests::{setup_test_database, teardown_test_database},
    utils::random::generate_guid,
};

async fn setup_db() -> (Pool<MySql>, String) {
    let db_name = &generate_guid(8);
    let pool = setup_test_database(&db_name).await;

    let student_id = Student::create(
        &pool,
        "first_name",
        "test@example.com",
        "hashed_password123",
        true,
        false,
        false,
    )
    .await
    .unwrap()
    .last_insert_id() as i32;

    VerificationCode::create_or_update_student_code(&pool, student_id, "12345678")
        .await
        .unwrap();

    (pool, db_name.clone())
}

#[actix_web::test]
async fn test_verify_email_successful() {
    let (pool, db_name) = setup_db().await;
    let mut app = test::init_service(
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(verify_email_post),
    )
    .await;

    let verify_email_model = VerifyEmailModel {
        email: "test@example.com".to_string(),
        verification_code: "12345678".to_string(),
    };

    let req = test::TestRequest::post()
        .uri("/verify-email")
        .set_json(&verify_email_model)
        .to_request();

    let resp = test::call_service(&mut app, req).await;
    println!("****{:?}****", resp);
    assert_eq!(resp.status(), http::StatusCode::OK);
    teardown_test_database(&pool, &db_name).await.unwrap();
}

#[actix_web::test]
async fn test_verify_email_invalid_code() {
    let (pool, db_name) = setup_db().await;
    let mut app = test::init_service(
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(verify_email_post),
    )
    .await;

    let verify_email_model = VerifyEmailModel {
        email: "test@example.com".to_string(),
        verification_code: "wrongcode".to_string(),
    };

    let req = test::TestRequest::post()
        .uri("/verify-email")
        .set_json(&verify_email_model)
        .to_request();

    let resp = test::call_service(&mut app, req).await;
    assert_eq!(resp.status(), http::StatusCode::BAD_REQUEST);
    teardown_test_database(&pool, &db_name).await.unwrap();
}

#[actix_web::test]
async fn test_verify_email_not_found() {
    let (pool, db_name) = setup_db().await;
    let mut app = test::init_service(
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(verify_email_post),
    )
    .await;

    let verify_email_model = VerifyEmailModel {
        email: "nonexistent@example.com".to_string(),
        verification_code: "12345678".to_string(),
    };

    let req = test::TestRequest::post()
        .uri("/verify-email")
        .set_json(&verify_email_model)
        .to_request();

    let resp = test::call_service(&mut app, req).await;
    assert_eq!(resp.status(), http::StatusCode::NOT_FOUND);
    teardown_test_database(&pool, &db_name).await.unwrap();
}

#[actix_web::test]
async fn test_verify_email_code_not_found() {
    let (pool, db_name) = setup_db().await;
    let mut app = test::init_service(
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(verify_email_post),
    )
    .await;

    Student::create(
        &pool,
        "user2",
        "test2@example.com",
        "hashed_password123",
        true,
        false,
        false,
    )
    .await
    .unwrap();

    let verify_email_model = VerifyEmailModel {
        email: "another@example.com".to_string(),
        verification_code: "12345678".to_string(),
    };

    let req = test::TestRequest::post()
        .uri("/verify-email")
        .set_json(&verify_email_model)
        .to_request();

    let resp = test::call_service(&mut app, req).await;
    assert_eq!(resp.status(), http::StatusCode::NOT_FOUND);
    teardown_test_database(&pool, &db_name).await.unwrap();
}
