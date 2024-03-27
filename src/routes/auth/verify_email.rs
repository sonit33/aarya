use actix_web::{ get, post, web, HttpResponse, Responder };
use sqlx::MySqlPool;
use tera::{ Context, Tera };
use validator::Validate;

use crate::models::{
    auth::verify_email::VerifyEmailModel,
    database::{ student::Student, verification_code::VerificationCode },
    default_response::{ ActionType, DefaultResponseModel, ResponseAction },
};

#[post("/verify-email")]
pub async fn verify_email_post(
    pool: web::Data<MySqlPool>,
    model: web::Json<VerifyEmailModel>
) -> impl Responder {
    println!("***verify_email_post***");
    // Validate the request model
    if let Err(e) = model.validate() {
        return HttpResponse::BadRequest().json(DefaultResponseModel::<()> {
            message: format!("Validation error: {}", e),
            payload: (),
            action: ResponseAction {
                action_type: ActionType::Resolve,
                arg: "".to_string(),
            },
        });
    }

    let verify_email = model.into_inner();

    match Student::read_by_email(&pool, &verify_email.email).await {
        Ok(Some(mut student)) => {
            // Read the verification code for the student
            match VerificationCode::read_student_code(pool.get_ref(), student.student_id).await {
                Ok(Some(code)) if code.code == verify_email.verification_code => {
                    student.account_active = true;
                    student.email_verified = true;
                    match student.update(&pool).await {
                        Ok(_) =>
                            HttpResponse::Ok().json(DefaultResponseModel::<()> {
                                message: "Email verified successfully.".to_string(),
                                payload: (),
                                // Assuming redirect logic is determined client-side for simplicity
                                action: ResponseAction {
                                    action_type: ActionType::Redirect,
                                    arg: "/login".to_string(), // Placeholder
                                },
                            }),
                        Err(_) =>
                            HttpResponse::InternalServerError().json(DefaultResponseModel::<()> {
                                message: "Failed to update student record.".to_string(),
                                payload: (),
                                action: ResponseAction {
                                    action_type: ActionType::Resolve,
                                    arg: "".to_string(),
                                },
                            }),
                    }
                }
                Ok(Some(_)) =>
                    HttpResponse::BadRequest().json(DefaultResponseModel::<()> {
                        message: "Invalid verification code.".to_string(),
                        payload: (),
                        action: ResponseAction {
                            action_type: ActionType::Resolve,
                            arg: "resend_code".to_string(), // Indicate action to resend verification email
                        },
                    }),
                Ok(None) => {
                    eprintln!("Verification code not found.");
                    HttpResponse::NotFound().json(DefaultResponseModel::<()> {
                        message: "Verification code not found.".to_string(),
                        payload: (),
                        action: ResponseAction {
                            action_type: ActionType::Resolve,
                            arg: "".to_string(),
                        },
                    })
                }

                Err(e) => {
                    eprintln!("{:?}", e);
                    HttpResponse::InternalServerError().json(DefaultResponseModel::<()> {
                        message: "Error fetching verification code.".to_string(),
                        payload: (),
                        action: ResponseAction {
                            action_type: ActionType::Resolve,
                            arg: "".to_string(),
                        },
                    })
                }
            }
        }
        Ok(None) =>
            HttpResponse::NotFound().json(DefaultResponseModel::<()> {
                message: "Email address not registered.".to_string(),
                payload: (),
                action: ResponseAction {
                    action_type: ActionType::Resolve,
                    arg: "signup".to_string(), // Indicate action to navigate to signup page
                },
            }),
        Err(e) => {
            eprintln!("{:?}", e);
            HttpResponse::InternalServerError().json(DefaultResponseModel::<()> {
                message: "Database error.".to_string(),
                payload: (),
                action: ResponseAction {
                    action_type: ActionType::Resolve,
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
