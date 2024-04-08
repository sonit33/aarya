use aarya_routes::auth::forgot_password::forgot_password_email_post;
use aarya_utils::{email_sender::EmailSender, random::generate_guid};
use actix_web::{http::StatusCode, test, web, App};
use serde_json::json;

use crate::{setup_database, teardown_database};

#[actix_web::test]
async fn forgot_password_email_post_email_not_found() {
    let db_name = generate_guid(8);
    let mock_pool = setup_database(&db_name).await;
    let mock_email_sender = EmailSender {};
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(mock_pool.clone()))
            .app_data(web::Data::new(mock_email_sender.clone()))
            .service(forgot_password_email_post),
    )
    .await;

    let req_body = json!({"email_address": "nonexistent@example.com"});
    let req = test::TestRequest::post()
        .uri("/forgot-password")
        .set_json(&req_body)
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::NOT_FOUND);
    teardown_database(&mock_pool, &db_name).await.unwrap();
}

#[actix_web::test]
async fn forgot_password_email_post_verification_code_error() {
    // You should mock VerificationCode::create_or_update_student_code to return an error
    // Assume it's done here
}

#[actix_web::test]
async fn forgot_password_email_post_email_send_error() {
    // You should mock EmailSender::send_email to return an error
    // Assume it's done here
}

#[actix_web::test]
async fn forgot_password_email_post_success() {
    let db_name = generate_guid(8);
    let mock_pool = setup_database(&db_name).await;
    let mock_email_sender = EmailSender {};
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(mock_pool.clone()))
            .app_data(web::Data::new(mock_email_sender.clone()))
            .service(forgot_password_email_post),
    )
    .await;

    let req_body = json!({"email_address": "jon@abc.com"});
    let req = test::TestRequest::post()
        .uri("/forgot-password")
        .set_json(&req_body)
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::OK);
    teardown_database(&mock_pool, &db_name).await.unwrap();
}
