use std::sync::Arc;

use actix_web::{ get, HttpResponse, post, Responder, web, web::Query };
use serde::{ Deserialize, Serialize };
use sqlx::MySqlPool;
use tera::{ Context, Tera };

use crate::models::database::student::Student;
use crate::models::database::verification_code::VerificationCode;
use crate::models::default_response::{ ActionType, DefaultResponseModel, ResponseAction };
use crate::utils::crc32_hasher;
use crate::utils::email_sender::EmailSender;
use crate::utils::random::generate_guid;

#[derive(Serialize, Deserialize)]
pub struct EmailSentModel {
    pub e: String,
}

// emails a password reset link to the user
#[post("/forgot-password/email-address")]
pub async fn forgot_password_email_post(
    pool: web::Data<MySqlPool>,
    email_sender: web::Data<Arc<EmailSender>>,
    email_address: web::Json<String>
) -> impl Responder {
    match Student::read_by_email(&pool, &email_address).await {
        Ok(result) => {
            match result {
                None => {
                    HttpResponse::NotFound().json(DefaultResponseModel::<String> {
                        json_payload: format!("Email address {} not found", &email_address),
                        action: ResponseAction {
                            action_type: ActionType::HandleError,
                            arg: "".to_string(),
                        },
                    })
                }
                Some(student) => {
                    // email address found
                    let v_code = generate_guid(8);
                    // save the verification code
                    match
                        VerificationCode::create_or_update_student_code(
                            &pool,
                            student.student_id,
                            &v_code
                        ).await
                    {
                        Ok(_) => {
                            // send the verification code in an email
                            // generate a link to reset password e.g. /reset-password?e=<email-hash>&c=<verification-code-hash>&t=<timestamp>
                            let reset_password_link = format!(
                                "/reset-password?e={}&c={},&t={}",
                                student.email_address_hash,
                                crc32_hasher::hash(&v_code).as_str(),
                                time::OffsetDateTime::now_utc().unix_timestamp().to_string()
                            );
                            match
                                email_sender.send_email(
                                    "postmaster@aarya.ai",
                                    &student.email_address,
                                    format!(
                                        "{}'s password reset link",
                                        &student.first_name
                                    ).as_str(),
                                    &reset_password_link
                                ).await
                            {
                                Ok(_) => {
                                    // send 200 and ask to redirect to the next page
                                    HttpResponse::Ok().json(DefaultResponseModel::<String> {
                                        json_payload: format!(
                                            "Aarya sent a verification code to your email address: [{}]",
                                            &email_address
                                        ),
                                        action: ResponseAction {
                                            action_type: ActionType::Redirect,
                                            arg: format!(
                                                "/forgot-password/email-sent?e={}",
                                                student.email_address_hash
                                            ),
                                        },
                                    })
                                }
                                Err(e) =>
                                    HttpResponse::InternalServerError().json(
                                        DefaultResponseModel::<String> {
                                            json_payload: format!("Error sending email: {}", e),
                                            action: ResponseAction {
                                                action_type: ActionType::HandleError,
                                                arg: "".to_string(),
                                            },
                                        }
                                    ),
                            }
                        }
                        Err(e) =>
                            HttpResponse::InternalServerError().json(
                                DefaultResponseModel::<String> {
                                    json_payload: format!("Error generating the verification code: {}", e),
                                    action: ResponseAction {
                                        action_type: ActionType::HandleError,
                                        arg: "".to_string(),
                                    },
                                }
                            ),
                    }
                }
            }
        }
        Err(e) =>
            HttpResponse::InternalServerError().json(DefaultResponseModel::<String> {
                json_payload: format!("Server error: {}", e),
                action: ResponseAction {
                    action_type: ActionType::HandleError,
                    arg: "".to_string(),
                },
            }),
    }
}

#[get("/forgot-password/email-address")]
pub async fn forgot_password_email_get(tera: web::Data<Tera>) -> impl Responder {
    let mut context = Context::new();
    context.insert("title", &"Forgot password?");

    match tera.render("auth/forgot-password/email-address.html", &context) {
        Ok(body) => HttpResponse::Ok().content_type("text/html").body(body),
        Err(e) => {
            println!("Error rendering template: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[get("/forgot-password/email-sent")]
pub async fn forgot_password_email_sent_get(
    tera: web::Data<Tera>,
    pool: web::Data<MySqlPool>,
    query: web::Query<EmailSentModel>
) -> impl Responder {
    let mut context = Context::new();
    context.insert("title", &"Forgot password?");

    // retrieve the student record from email hash
    match Student::read_by_email(&pool, &query.0.e).await {
        Ok(s) => {
            match s {
                Some(student) => {
                    context.insert("email_address", &student.email_address);
                    match tera.render("auth/forgot-password/email-sent.html", &context) {
                        Ok(body) => HttpResponse::Ok().content_type("text/html").body(body),
                        Err(e) => {
                            println!("Error rendering template: {}", e);
                            HttpResponse::InternalServerError().finish()
                        }
                    }
                }
                None => { HttpResponse::NotFound().body("404: not found") }
            }
        }
        Err(_) => HttpResponse::InternalServerError().body("500: server error"),
    }
}
