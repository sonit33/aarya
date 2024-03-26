use actix_web::{App, http, test};
use serde_json::json;

#[actix_web::test]
async fn test_signup_validation_error() {
	let mut app = test::init_service(App::new().route("/signup", web::post().to(signup_post))).await;
	let req_body = json!({
        "user_id": "1", // Assuming this is invalid for your validation rules
        // Add other fields here, intentionally missing or invalid to trigger validation errors
    });

	let req = test::TestRequest::post()
		.uri("/signup")
		.set_json(&req_body)
		.to_request();
	let resp = test::call_service(&mut app, req).await;

	assert_eq!(resp.status(), http::StatusCode::BAD_REQUEST);

	let result: DefaultResponseModel<()> = test::read_body_json(resp).await;
	assert_eq!(result.action.action_type, ActionType::Resolve);
	assert!(result.message.contains("Validation error:"));
}
