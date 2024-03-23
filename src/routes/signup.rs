use actix_web::{get, HttpResponse, post, Responder, web};
use tera::{Context, Tera};
use validator::Validate;

use crate::models::default_response::{ActionType, DefaultResponseModel, ResponseAction};
use crate::models::signup::SignupModel;

#[post("/signup")]
pub async fn signup_post(model: web::Json<SignupModel>) -> impl Responder {
    // let mut response: ApiResponseModel<String>;
    // validate the input
    match model.0.validate() {
        Ok(_) => {
            // generate a verification code
            // email the verification code
            // save the user to database
            // send a redirect to verify page

            HttpResponse::Ok().json(DefaultResponseModel {
                message: "new user registered".to_string(),
                payload: "123456".to_string(), // user_id
                action: ResponseAction {
                    action_type: ActionType::Redirect,
                    arg: "/verify".to_string(),
                },
            })
        }
        Err(e) => {
            HttpResponse::BadRequest().json(DefaultResponseModel {
                message: "validation error".to_string(),
                payload: format!("{:?}", e),
                action: ResponseAction {
                    action_type: ActionType::Resolve,
                    arg: "".to_string(),
                },
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