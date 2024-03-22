use actix_web::{HttpResponse, Responder, web};

use crate::model_api_response::ApiResponseModel;
use crate::model_signup::SignupModel;

pub async fn signup_post(model: web::Json<SignupModel>) -> impl Responder {
    // Access the SignupModel fields
    // let display_name = &model.display_name;
    // let email = &model.email;
    // let password = &model.password;
    // let confirm_password = &model.confirm_password;
    // let over_13 = model.over_13;

    // Create a custom response
    let response = ApiResponseModel {
        success: true,
        message: format!("Signup successful for user: {}", model.display_name),
        status_code: Default::default(),
        payload: model,
    };

    // Return the JSON response
    HttpResponse::Ok().json(response)
}