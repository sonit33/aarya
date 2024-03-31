use std::sync::Arc;

use actix_web::{ get, HttpResponse, post, Responder, web };
use sqlx::MySqlPool;
use tera::{ Context, Tera };
use validator::Validate;

use crate::models::auth::signup::SignupModel;
use crate::models::database::student::Student;
use crate::models::default_response::{ ActionType, DefaultResponseModel, ResponseAction };
use crate::utils::email_sender::EmailSender;
use crate::utils::hasher;

#[post("/signup")]
pub async fn signup_post(
    pool: web::Data<Arc<MySqlPool>>,
    email_sender: web::Data<Arc<EmailSender>>,
    model: web::Json<SignupModel>
) -> impl Responder {
    // Validate the SignupModel
    if let Err(e) = model.validate() {
        return HttpResponse::BadRequest().json(DefaultResponseModel::<String> {
            json_payload: format!("Validation error: {}", e),
            action: ResponseAction {
                action_type: ActionType::HandleError,
                arg: "".to_string(),
            },
        });
    }

    let signup = model.into_inner();

    // Transform SignupModel into Student
    let student = Student {
        student_id: Some(-1),
        first_name: signup.display_name.clone(),
        email_address: signup.email.clone(),
        id_hash: "".to_string(),
        email_hash: "".to_string(),
        password: signup.password,
        over_13: signup.over_13,
        email_verified: false,
        account_active: false,
        added_timestamp: None,
        updated_timestamp: None,
        deleted_timestamp: None,
    };

    // let student_id;

    // Save the Student in the database
    match
        Student::create(
            &pool,
            &student.first_name,
            &student.email_address,
            hasher::cook_hash(&student.password).unwrap().as_str(),
            student.over_13,
            false, // email_verified as false
            false // account_active as false
        ).await
    {
        Ok(_) => {
            // student_id = s.last_insert_id();
        }
        Err(e) => {
            return HttpResponse::InternalServerError().json(DefaultResponseModel::<String> {
                json_payload: format!("Failed to create student: {}", e),
                action: ResponseAction {
                    action_type: ActionType::HandleError,
                    arg: "".to_string(),
                },
            });
        }
    }

    // Generate a verification code
    // let verification_code = generate_guid(8); // Placeholder for generated code
    // generate a link that completes the email verification

    let link = "/verify-email/<email-hash>::<code-hash>::<timestamp>";

    // Email the verification code
    match
        email_sender.send_email(
            "admin@aarya.ai",
            &student.email_address,
            format!("{} activate your Aarya account", &student.first_name).as_str(),
            &link
        ).await
    {
        Ok(_) => {}
        Err(e) => {
            return HttpResponse::InternalServerError().json(DefaultResponseModel::<String> {
                json_payload: format!("Failed to send verification email: {}", e),
                action: ResponseAction {
                    action_type: ActionType::HandleError,
                    arg: "".to_string(),
                },
            });
        }
    }

    // Success: send a 200 HTTP response
    HttpResponse::Ok().json(DefaultResponseModel::<String> {
        json_payload: "Signup successful. Please check your email to verify your account.".to_string(),
        action: ResponseAction {
            action_type: ActionType::Redirect,
            arg: "/login".to_string(),
        },
    })
}

#[get("/signup")]
pub async fn signup_get(tera: web::Data<Tera>) -> impl Responder {
    let mut context = Context::new();
    context.insert("title", &"Signup for Aarya");

    match tera.render("auth/signup.html", &context) {
        Ok(body) => HttpResponse::Ok().content_type("text/html").body(body),
        Err(e) => {
            println!("Error rendering template: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}
