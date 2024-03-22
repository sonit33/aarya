use actix_web::{get, HttpResponse, post, Responder, web};
use tera::{Context, Tera};
use validator::Validate;

use crate::model_api_response::ApiResponseModel;
use crate::model_signup::SignupModel;

#[post("/signup")]
pub async fn signup_post(model: web::Json<SignupModel>) -> impl Responder {
    // let mut response: ApiResponseModel<String>;
    // validate the input
    match model.0.validate() {
        Ok(_) => {
            HttpResponse::Ok().json(ApiResponseModel {
                message: "new user registered".to_string(),
                payload: "123456".to_string(), // user_id
            })
        }
        Err(e) => {
            HttpResponse::BadRequest().json(ApiResponseModel {
                message: "validation error".to_string(),
                payload: format!("{:?}", e),
            })
        }
    }
}

#[get("/signup")]
pub async fn signup_get(tera: web::Data<Tera>) -> impl Responder {
    let mut context = Context::new();
    context.insert("title", &"Signup for Aarya");

    match tera.render("auth_signup.html.tera", &context) {
        Ok(body) => HttpResponse::Ok().content_type("text/html").body(body),
        Err(e) => {
            println!("Error rendering template: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}