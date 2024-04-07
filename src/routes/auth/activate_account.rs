use actix_web::{get, post, web, HttpResponse, Responder};
use sqlx::MySqlPool;
use tera::{Context, Tera};
use validator::Validate;

use crate::{
    bad_request,
    models::{
        auth::{
            base64_qs_model::{extract_values, Base64QuerystringModel},
            email_qs_model::EmailSentModel,
            verify_email::VerifyEmailModel,
        },
        database::student::Student,
        default_response::{ActionType, DefaultResponseModel},
    },
    not_found, ok_action, render_template, server_error,
    utils::{email_sender::EmailSender, encoder::UrlEncoderDecoder, timestamps},
};

#[post("/activate-account")]
pub async fn activate_account_post(
    pool: web::Data<MySqlPool>,
    email_sender: web::Data<EmailSender>,
    model: web::Json<VerifyEmailModel>,
) -> impl Responder {
    // validate the model
    if let Err(e) = model.validate() {
        bad_request!(format!("Validation error: {}", e));
    }

    let mut student = Student::new();
    student.email_address = model.email_address.to_string();

    match student.read_by_email(&pool).await {
        Ok(result) => {
            match result {
                None => not_found!(format!("Email address {} not found", &model.email_address)),
                Some(student) => {
                    // email address found
                    // generate a url-encoded link to reset password e.g. /reset-password?e=<email-hash>&t=<timestamp>
                    let reset_password_link = format!(
                        "/account-confirmation?q={}",
                        UrlEncoderDecoder::encode(
                            format!(
                                "e={}&t={}",
                                student.email_hash,
                                time::OffsetDateTime::now_utc().unix_timestamp().to_string()
                            )
                            .as_str()
                        )
                    );
                    match email_sender
                        .send_email(
                            "postmaster@aarya.ai",
                            &student.email_address,
                            format!("{}'s password reset link", &student.first_name).as_str(),
                            &reset_password_link,
                        )
                        .await
                    {
                        Ok(_) => ok_action!(
                            ActionType::Redirect,
                            format!("/activate-account/email-sent?e={}", student.email_hash)
                        ),
                        Err(e) => server_error!("Error sending email", e),
                    }
                }
            }
        }
        Err(e) => server_error!("Server error", e),
    }
}

#[get("/activate-account")]
pub async fn activate_account_get(tera: web::Data<Tera>) -> impl Responder {
    let mut context = Context::new();
    context.insert("title", &"Verify your email address");

    render_template!(&tera, "auth/activate-account/index.html", &context)
}

#[get("/account-confirmation")]
pub async fn account_activate_get(
    tera: web::Data<Tera>,
    pool: web::Data<MySqlPool>,
    query: web::Query<Base64QuerystringModel>,
) -> impl Responder {
    let mut context = Context::new();
    context.insert("title", &"Activate your account");

    let values = extract_values(&query.q);

    log::debug!("{:?}", values);

    // use e = email_hash, and t=timestamp to determine the validity of this link (T+1 day)

    let email_hash = values.0.unwrap();
    let timestamp = values.1.unwrap();

    let mut student = Student::new();
    student.email_hash = email_hash.to_string();

    match student.read_by_email(&pool).await {
        Ok(s) => match timestamp.parse::<i64>() {
            Ok(t) => {
                let days = timestamps::difference_in_days(timestamps::get_unix_timestamp(), t);
                log::debug!("{} {} {}", t, timestamps::get_unix_timestamp(), days);
                if days == 0 {
                    let mut student = s.unwrap();
                    student.account_active = true;
                    student.email_verified = true;
                    match student.update_account(&pool).await {
                        Ok(_) => {
                            context.insert("email_hash", email_hash);
                            render_template!(
                                &tera,
                                "auth/activate-account/confirmation.html",
                                &context
                            )
                        }
                        Err(_) => {
                            context.insert("error", "activation failed");
                            render_template!(
                                &tera,
                                "auth/activate-account/confirmation.html",
                                &context
                            )
                        }
                    }
                } else {
                    context.insert("error", "link has expired");
                    render_template!(&tera, "auth/activate-account/confirmation.html", &context)
                }
            }
            Err(_) => {
                context.insert("error", "invalid url");
                render_template!(&tera, "auth/activate-account/confirmation.html", &context)
            }
        },
        Err(_) => {
            context.insert("error", "email not found");
            render_template!(&tera, "auth/activate-account/confirmation.html", &context)
        }
    }
}

#[get("/activate-account/email-sent")]
pub async fn activate_account_email_sent_get(
    tera: web::Data<Tera>,
    pool: web::Data<MySqlPool>,
    query: web::Query<EmailSentModel>,
) -> impl Responder {
    let mut context = Context::new();
    context.insert("title", &"Activate your account");

    log::debug!("{:?}", &query);

    let mut student = Student::new();
    student.email_hash = query.e.to_string();

    // retrieve the student record from email hash
    match student.read_by_email(&pool).await {
        Ok(s) => match s {
            Some(student) => {
                context.insert("email_address", &student.email_address);
                log::debug!("{:?}", &context);
                render_template!(&tera, "auth/activate-account/email-sent.html", &context)
            }
            None => {
                context.insert("error", "not found");
                render_template!(&tera, "auth/activate-account/email-sent.html", &context)
            }
        },
        Err(e) => {
            context.insert("error", "server error");
            log::debug!("{:?}", e);
            render_template!(&tera, "auth/activate-account/email-sent.html", &context)
        }
    }
}
