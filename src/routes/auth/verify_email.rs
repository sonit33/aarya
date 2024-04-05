use actix_web::{ get, HttpResponse, post, Responder, web };
use sqlx::MySqlPool;
use tera::{ Context, Tera };
use validator::Validate;

use crate::{
    bad_request,
    models::{
        auth::verify_email::VerifyEmailModel,
        database::student::Student,
        default_response::{ ActionType, DefaultResponseModel },
    },
    not_found,
    ok_action,
    render_template,
    server_error,
};

#[post("/verify-email")]
pub async fn verify_email_post(
    pool: web::Data<MySqlPool>,
    model: web::Json<VerifyEmailModel>
) -> impl Responder {
    // Validate the request model
    if let Err(e) = model.validate() {
        return bad_request!(e);
    }

    let verify_email = model.into_inner();

    match Student::read_by_email(&pool, &verify_email.email_address).await {
        Ok(Some(mut student)) => {
            // Read the verification code for the student
            student.account_active = true;
            student.email_verified = true;
            match student.update(&pool).await {
                Ok(_) => ok_action!(ActionType::Redirect, "/login"),
                Err(e) => server_error!(format!("Failed to update student: [{}]", e)),
            }
        }
        Ok(None) => not_found!("Email address not registered."),

        Err(e) => server_error!(format!("Database error. [{}]", e)),
    }
}

#[get("/verify-email")]
pub async fn verify_email_get(tera: web::Data<Tera>) -> impl Responder {
    let mut context = Context::new();
    context.insert("title", &"Verify your email address");

    render_template!(&tera, "auth/verify-email.html", &context)
}
