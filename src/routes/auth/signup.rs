use std::sync::Arc;

use actix_web::{get, post, web, HttpResponse, Responder};
use sqlx::MySqlPool;
use tera::{Context, Tera};
use validator::Validate;

use crate::models::auth::signup::SignupModel;
use crate::models::database::student::Student;
use crate::models::default_response::{ActionType, DefaultResponseModel};
use crate::utils::email_sender::EmailSender;
use crate::utils::encoder::UrlEncoderDecoder;
use crate::{bad_request, ok_action, render_template, server_error};

#[post("/signup")]
pub async fn signup_post(
    pool: web::Data<Arc<MySqlPool>>,
    email_sender: web::Data<Arc<EmailSender>>,
    model: web::Json<SignupModel>,
) -> impl Responder {
    // Validate the SignupModel
    if let Err(e) = model.validate() {
        return bad_request!(format!("Validation error: {}", e));
    }

    let signup = model.into_inner();

    // Transform SignupModel into Student
    let mut student = Student {
        student_id: Some(0),
        first_name: signup.display_name.clone(),
        email_address: signup.email.clone(),
        id_hash: "not-set".to_string(),
        email_hash: "not-set".to_string(),
        pass_hash: Some(signup.password),
        over_13: signup.over_13,
        email_verified: false,
        account_active: false,
        added_timestamp: None,
    };

    let student_id = match student.create(&pool).await {
        Ok(r) => r.last_insert_id(),
        Err(e) => return server_error!("Failed to create new user", e),
    };

    // retrieve the student for email_hash and added_timestamp
    student.student_id = Some(student_id as u32);
    let created_student = match student.read(&pool).await {
        Ok(r) => r.unwrap(),
        Err(e) => return server_error!("Failed to retrieve new user", e),
    };

    let link = format!(
        "/activate-account/{}",
        UrlEncoderDecoder::encode(
            format!(
                "e={}&t={}",
                created_student.email_hash,
                created_student.added_timestamp.unwrap()
            )
            .as_str()
        )
    );

    // Email the verification code
    match email_sender
        .send_email(
            "admin@aarya.ai",
            &student.email_address,
            format!("{} activate your Aarya account", &student.first_name).as_str(),
            &link,
        )
        .await
    {
        Ok(_) => {}
        Err(e) => {
            return server_error!("Failed to send verification email", e);
        }
    }

    ok_action!(ActionType::Redirect, "/login")
}

#[get("/signup")]
pub async fn signup_get(tera: web::Data<Tera>) -> impl Responder {
    let mut context = Context::new();
    context.insert("title", &"Signup for Aarya");

    render_template!(&tera, "auth/signup.html", &context)
}
