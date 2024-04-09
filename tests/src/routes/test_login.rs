use actix_web::{http::StatusCode, test, web, App};

use crate::{
    models::{auth::login::LoginModel, database::student::Student},
    routes::auth::login::login_post,
    tests::{setup_test_database, teardown_test_database},
    utils::{hasher, random::generate_guid},
};

#[actix_web::test]
async fn test_login_success() {
    let db_name = generate_guid(8);
    let mock_pool = setup_test_database(&db_name).await;

    let mut app = test::init_service(
        App::new()
            .app_data(web::Data::new(mock_pool.clone()))
            .service(login_post),
    )
    .await;

    // insert a student record so that it can be found
    Student::create(
        &mock_pool,
        "first_name",
        "test@example.com",
        hasher::cook_hash("password123").unwrap().as_str(),
        true,
        true,
        true,
    )
    .await
    .unwrap();

    let login_model = LoginModel {
        email: "test@example.com".to_string(),
        password: "password123".to_string(),
        stay_signed_in: true,
    };

    let req = test::TestRequest::post()
        .uri("/login")
        .set_json(&login_model)
        .to_request();

    let resp = test::call_service(&mut app, req).await;
    assert_eq!(resp.status(), StatusCode::OK);

    teardown_test_database(&mock_pool, &db_name).await.unwrap();
}

#[actix_web::test]
async fn test_login_invalid_credentials() {
    let db_name = generate_guid(8);
    let mock_pool = setup_test_database(&db_name).await;

    let mut app = test::init_service(
        App::new()
            .app_data(web::Data::new(mock_pool.clone()))
            .service(login_post),
    )
    .await;

    Student::create(
        &mock_pool,
        "first_name",
        "nonexistent@example.com",
        hasher::cook_hash("password123").unwrap().as_str(),
        true,
        true,
        true,
    )
    .await
    .unwrap();

    let login_model = LoginModel {
        email: "nonexistent@example.com".to_string(),
        password: "wrongpassword".to_string(),
        stay_signed_in: false,
    };

    let req = test::TestRequest::post()
        .uri("/login")
        .set_json(&login_model)
        .to_request();

    let resp = test::call_service(&mut app, req).await;
    assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);

    teardown_test_database(&mock_pool, &db_name).await.unwrap();
}

// Additional tests for handling validation errors, database errors, inactive account, etc.

// Teardown or cleanup database if necessary
