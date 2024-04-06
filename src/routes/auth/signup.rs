use std::sync::Arc;

use actix_web::{ get, HttpResponse, post, Responder, web };
use sqlx::MySqlPool;
use tera::{ Context, Tera };
use validator::Validate;

use crate::{ bad_request, ok_action, render_template, server_error };
use crate::models::auth::signup::SignupModel;
use crate::models::database::student::Student;
use crate::models::default_response::{ ActionType, DefaultResponseModel };
use crate::utils::email_sender::EmailSender;

#[post("/signup")]
pub async fn signup_post(
    pool: web::Data<Arc<MySqlPool>>,
    email_sender: web::Data<Arc<EmailSender>>,
    model: web::Json<SignupModel>
) -> impl Responder {
    // Validate the SignupModel
    if let Err(e) = model.validate() {
        return bad_request!(format!("Validation error: {}", e));
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
    //TODO
    // match
    //     Student::create(
    //         &pool,
    //         &student.first_name,
    //         &student.email_address,
    //         hasher::cook_hash(&student.password).unwrap().as_str(),
    //         student.over_13,
    //         false, // email_verified as false
    //         false // account_active as false
    //     ).await
    // {
    //     Ok(_) => {
    //         // student_id = s.last_insert_id();
    //     }
    //     Err(e) => server_error!(format!("Failed to create student: {}", e)),
    // }

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
            return server_error!(format!("Failed to send verification email: [{e}]"));
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
