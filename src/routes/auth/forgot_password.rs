use actix_web::{ get, post, web, HttpResponse, Responder };
use serde::{ Deserialize, Serialize };
use sqlx::MySqlPool;
use tera::{ Context, Tera };
use validator::Validate;

use crate::{ bad_request, not_found, ok_action, render_template, server_error };
use crate::models::auth::verify_email::VerifyEmailModel;
use crate::models::database::student::Student;
use crate::{
    models::default_response::{ ActionType, DefaultResponseModel },
    utils::encoder::UrlEncoderDecoder,
};
use crate::utils::email_sender::EmailSender;

#[derive(Serialize, Deserialize, Debug)]
pub struct EmailSentModel {
    pub e: String,
}

// emails a password reset link to the user
#[post("/forgot-password")]
pub async fn forgot_password_email_post(
    pool: web::Data<MySqlPool>,
    email_sender: web::Data<EmailSender>,
    model: web::Json<VerifyEmailModel>
) -> impl Responder {
    // validate the model
    if let Err(e) = model.validate() {
        bad_request!(format!("Validation error: {}", e));
    }
    match Student::read_by_email(&pool, &model.email_address).await {
        Ok(result) => {
            match result {
                None => not_found!(format!("Email address {} not found", &model.email_address)),
                Some(student) => {
                    // email address found
                    // generate a url-encoded link to reset password e.g. /reset-password?e=<email-hash>&t=<timestamp>
                    let reset_password_link = format!(
                        "/reset-password?q={}",
                        UrlEncoderDecoder::encode(
                            format!(
                                "e={}&t={}",
                                student.email_hash,
                                time::OffsetDateTime::now_utc().unix_timestamp().to_string()
                            ).as_str()
                        )
                    );
                    match
                        email_sender.send_email(
                            "postmaster@aarya.ai",
                            &student.email_address,
                            format!("{}'s password reset link", &student.first_name).as_str(),
                            &reset_password_link
                        ).await
                    {
                        Ok(_) =>
                            ok_action!(
                                ActionType::Redirect,
                                format!("/forgot-password/email-sent?e={}", student.email_hash)
                            ),
                        Err(e) => server_error!(format!("Error sending email: {}", e)),
                    }
                }
            }
        }
        Err(e) => server_error!(format!("Server error: {}", e)),
    }
}

#[get("/forgot-password")]
pub async fn forgot_password_get(tera: web::Data<Tera>) -> impl Responder {
    let mut context = Context::new();
    context.insert("title", &"Forgot password?");

    render_template!(&tera, "auth/forgot-password/verify-email.html", &context)
}

#[get("/forgot-password/email-sent")]
pub async fn forgot_password_email_sent_get(
    tera: web::Data<Tera>,
    pool: web::Data<MySqlPool>,
    query: web::Query<EmailSentModel>
) -> impl Responder {
    let mut context = Context::new();
    context.insert("title", &"Forgot password?");

    log::debug!("{:?}", &query);

    // retrieve the student record from email hash
    match Student::read_by_email_hash(&pool, &query.e).await {
        Ok(s) => {
            match s {
                Some(student) => {
                    context.insert("email_address", &student.email_address);
                    log::debug!("{:?}", &context);
                    render_template!(&tera, "auth/forgot-password/email-sent.html", &context)
                }
                None => {
                    context.insert("error", "not found");
                    render_template!(&tera, "auth/forgot-password/email-sent.html", &context)
                }
            }
        }
        Err(e) => {
            context.insert("error", "server error");
            log::debug!("{:?}", e);
            render_template!(&tera, "auth/forgot-password/email-sent.html", &context)
        }
    }
}
