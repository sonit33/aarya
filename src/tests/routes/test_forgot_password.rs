use actix_web::{ test, App, http::StatusCode };
use serde_json::json;

use crate::{
    routes::auth::forgot_password::forgot_password_email_post,
    tests::{ setup_database, teardown_database },
    utils::{ email_sender::EmailSender, random::generate_guid },
};

#[actix_web::test]
async fn forgot_password_email_post_path_not_found() {
    let db_name = generate_guid(8);
    let mock_pool = setup_database(&db_name).await;
    let mock_email_sender = EmailSender {};
    let app = test::init_service(
        App::new()
            .app_data(mock_pool.clone())
            .app_data(mock_email_sender.clone())
            .service(forgot_password_email_post)
    ).await;

    let req_body = json!({"email_address": "nonexistent@example.com"});
    let req = test::TestRequest
        ::post()
        .uri("/forgot-password/email-address")
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
            .app_data(mock_pool.clone())
            .app_data(mock_email_sender.clone())
            .service(forgot_password_email_post)
    ).await;

    let req_body = json!({"email_address": "existing@example.com"});
    let req = test::TestRequest
        ::post()
        .uri("/forgot-password/email-address")
        .set_json(&req_body)
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::OK);
    teardown_database(&mock_pool, &db_name).await.unwrap();
}
