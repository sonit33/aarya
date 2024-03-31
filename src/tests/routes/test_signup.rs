use std::sync::Arc;

use actix_web::{ http::StatusCode, test, web, App };
use serde_json::json;

use crate::{
    routes::auth::signup::signup_post,
    tests::{ setup_database, teardown_database },
    utils::{ email_sender::EmailSender, random::generate_guid },
};

#[actix_web::test]
async fn test_signup_post_success() {
    let db_name = generate_guid(8);
    let mock_pool = setup_database(&db_name).await;
    let mock_email_sender = EmailSender {};

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(Arc::new(mock_pool.clone())))
            .app_data(web::Data::new(Arc::new(mock_email_sender)))
            .service(signup_post)
    ).await;

    let signup_model =
        json!({
        "user_id": "testuser123",
        "display_name": "Test User",
        "email": "test@example.com",
        "password": "securepassword",
        "confirm_password": "securepassword",
        "over_13": true,
        "verification_code": "12345678",
    });

    let req = test::TestRequest::post().uri("/signup").set_json(&signup_model).to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::OK);
    teardown_database(&mock_pool, &db_name).await.unwrap();
}

#[actix_web::test]
async fn test_signup_post_validation_failure() {
    let db_name = generate_guid(8);
    let mock_pool = setup_database(&db_name).await;

    let mock_email_sender = EmailSender {}; // Define this function to setup a mock email sender

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(Arc::new(mock_pool.clone())))
            .app_data(web::Data::new(Arc::new(mock_email_sender)))
            .service(signup_post)
    ).await;

    let invalid_signup_model =
        json!({
        "user_id": "tu",
        "display_name": "T",
        "email": "notanemail",
        "password": "pwd",
        "confirm_password": "pwd",
        "over_13": false,
        "verification_code": "1234",
    });

    let req = test::TestRequest::post().uri("/signup").set_json(&invalid_signup_model).to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
    teardown_database(&mock_pool, &db_name).await.unwrap();
}
