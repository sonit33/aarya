#[cfg(test)]
pub mod test_signup {
    use actix_web::{App, test, web};

    use crate::model_api_response::ApiResponseModel;
    use crate::model_signup::SignupModel;
    use crate::route_signup::signup_post;

    #[actix_web::test]
    async fn test_signup_route() {
        // Create a test instance of the Actix web app
        let mut app = test::init_service(
            App::new()
                .route("/signup", web::post().to(signup_post))
        ).await;

        // Create a sample SignupModel payload
        let signup_payload = SignupModel {
            id: "123".to_string(),
            display_name: "John Doe".to_string(),
            email: "john@example.com".to_string(),
            password: "password123".to_string(),
            confirm_password: "password123".to_string(),
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
        let resp_json: ApiResponseModel<SignupModel> = test::read_body_json(resp).await;
        assert_eq!(resp_json.success, true);
        assert_eq!(resp_json.message, "Signup successful for user: John Doe");
    }
}