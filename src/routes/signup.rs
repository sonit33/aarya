use std::sync::Arc;

use actix_web::{get, HttpResponse, post, Responder, web};
use sqlx::MySqlPool;
use tera::{Context, Tera};
use validator::Validate;

use crate::models::database::student::Student;
use crate::models::default_response::{ActionType, DefaultResponseModel, ResponseAction};
use crate::models::signup::SignupModel;
use crate::utils::email_sender::EmailSender;
use crate::utils::random::generate_guid;

#[post("/signup")]
async fn signup_post(
	pool: web::Data<Arc<MySqlPool>>,
	email_sender: web::Data<Arc<EmailSender>>,
	model: web::Json<SignupModel>,
) -> impl Responder {
	// Validate the SignupModel
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

	let signup = model.into_inner();

	// Transform SignupModel into Student
	let student = Student {
		student_id: 0,
		first_name: signup.display_name.clone(),
		email_address: signup.email.clone(),
		password: signup.password,
		over_13: signup.over_13,
		email_verified: false,
		account_active: false,
		added_timestamp: None,
		updated_timestamp: None,
		deleted_timestamp: None,
	};

	// Save the Student in the database
	match Student::create(
		&pool,
		&student.first_name,
		&student.email_address,
		&student.password,
		student.over_13 as i8,
		0, // email_verified as false
		0, // account_active as false
	).await {
		Ok(_) => {},
		Err(e) => {
			return HttpResponse::InternalServerError().json(DefaultResponseModel::<()> {
				message: format!("Failed to create student: {}", e),
				payload: (),
				action: ResponseAction {
					action_type: ActionType::Resolve,
					arg: "".to_string(),
				},
			});
		}
	};

	// Generate a verification code
	let verification_code = generate_guid(8); // Placeholder for generated code

	// Email the verification code
	match email_sender.send_email("admin@aarya.ai", &student.email_address, "Verification Code", &verification_code).await {
		Ok(_) => {},
		Err(e) => {
			return HttpResponse::InternalServerError().json(DefaultResponseModel::<()> {
				message: format!("Failed to send verification email: {}", e),
				payload: (),
				action: ResponseAction {
					action_type: ActionType::Resolve,
					arg: "".to_string(),
				},
			});
		}
	}

	// Success: send a 200 HTTP response
	HttpResponse::Ok().json(DefaultResponseModel::<()> {
		message: "Signup successful. Please check your email to verify your account.".to_string(),
		payload: (),
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

	match tera.render("auth_signup.html.tera", &context) {
		Ok(body) => HttpResponse::Ok().content_type("text/html").body(body),
		Err(e) => {
			println!("Error rendering template: {}", e);
			HttpResponse::InternalServerError().finish()
		}
	}
}