use actix_web::{ get, HttpResponse, post, Responder, web };
use sqlx::MySqlPool;
use tera::{ Context, Tera };
use validator::Validate;

use crate::models::{
    auth::verify_email::VerifyEmailModel,
    database::student::Student,
    default_response::{ ActionType, DefaultResponseModel, ResponseAction },
};

#[post("/verify-email")]
pub async fn verify_email_post(
    pool: web::Data<MySqlPool>,
    model: web::Json<VerifyEmailModel>
) -> impl Responder {
    // Validate the request model
    if let Err(e) = model.validate() {
        return HttpResponse::BadRequest().json(DefaultResponseModel::<String> {
            json_payload: format!("Validation error: {}", e),
            action: ResponseAction {
                action_type: ActionType::HandleError,
                arg: "".to_string(),
            },
        });
    }

    let verify_email = model.into_inner();

    match Student::read_by_email(&pool, &verify_email.email_address).await {
        Ok(Some(mut student)) => {
            // Read the verification code for the student
            student.account_active = true;
            student.email_verified = true;
            match student.update(&pool).await {
                Ok(_) =>
                    HttpResponse::Ok().json(DefaultResponseModel::<String> {
                        json_payload: "Email verified successfully.".to_string(),
                        // Assuming redirect logic is determined client-side for simplicity
                        action: ResponseAction {
                            action_type: ActionType::Redirect,
                            arg: "/login".to_string(), // Placeholder
                        },
                    }),
                Err(_) =>
                    HttpResponse::InternalServerError().json(DefaultResponseModel::<String> {
                        json_payload: "Failed to update student record.".to_string(),
                        action: ResponseAction {
                            action_type: ActionType::HandleError,
                            arg: "".to_string(),
                        },
                    }),
            }
        }
        Ok(None) =>
            HttpResponse::NotFound().json(DefaultResponseModel::<String> {
                json_payload: "Email address not registered.".to_string(),
                action: ResponseAction {
                    action_type: ActionType::HandleError,
                    arg: "signup".to_string(), // Indicate action to navigate to signup page
                },
            }),
        Err(e) => {
            eprintln!("{:?}", e);
            HttpResponse::InternalServerError().json(DefaultResponseModel::<String> {
                json_payload: "Database error.".to_string(),
                action: ResponseAction {
                    action_type: ActionType::HandleError,
                    arg: "".to_string(),
                },
            })
        }
    }
}

#[get("/verify-email")]
pub async fn verify_email_get(tera: web::Data<Tera>) -> impl Responder {
    let mut context = Context::new();
    context.insert("title", &"Verify your email address");

    match tera.render("auth/verify-email.html", &context) {
        Ok(body) => HttpResponse::Ok().content_type("text/html").body(body),
        Err(e) => {
            println!("Error rendering template: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}
