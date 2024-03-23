use actix_web::{App, test};
use dotenv::from_filename;

use crate::models::default_response::DefaultResponseModel;
use crate::models::signup::SignupModel;
use crate::routes::signup::signup_post;

#[tokio::test]
async fn test_signup_route() {
    from_filename(".env.dev").ok();
    // Create a test instance of the Actix web app
    let mut app = test::init_service(
        App::new().service(signup_post)
    ).await;

    // Create a sample SignupModel payload
    let signup_payload = SignupModel {
        user_id: "123".to_string(),
        display_name: "John Doe".to_string(),
        email: "john@example.com".to_string(),
        password: "password123".to_string(),
        confirm_password: "password123".to_string(),
        verification_code: "12345678".to_string(),
        over_13: true,
    };

    // Send a POST request to the /signup route with the payload
    let req = test::TestRequest::post()
        .uri("/signup")
        .set_json(&signup_payload)
        .to_request();

    let resp = test::call_service(&mut app, req).await;

    // Assert the response status code
    assert_eq!(resp.status(), 200);

    // Assert the response JSON
    let resp_json: DefaultResponseModel<String> = test::read_body_json(resp).await;
    println!("\x1b[31m{}\x1b[0m", resp_json.payload);
    assert_eq!(resp_json.message, "new user registered");
}