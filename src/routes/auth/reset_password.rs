use actix_web::{get, post, web, HttpResponse, Responder};
use log;
use sqlx::MySqlPool;
use tera::{Context, Tera};
use validator::Validate;

use crate::{
    bad_request,
    models::{
        auth::{
            base64_qs_model::{extract_values, Base64QuerystringModel},
            reset_password::ResetPasswordModel,
        },
        database::student::Student,
        default_response::{ActionType, DefaultResponseModel},
    },
    not_found, ok_action, render_template, server_error,
    utils::{hasher, timestamps},
};

#[get("/reset-password")]
pub async fn reset_password_get(
    tera: web::Data<Tera>,
    pool: web::Data<MySqlPool>,
    query: web::Query<Base64QuerystringModel>,
) -> impl Responder {
    let mut context = Context::new();
    context.insert("title", &"Reset your password");

    let values = extract_values(&query.q);

    log::debug!("{:?}", values);

    // use e = email_hash, and t=timestamp to determine the validity of this link (T+1 day)

    let email_hash = values.0.unwrap();
    let timestamp = values.1.unwrap();

    let mut student = Student::new();
    student.email_hash = email_hash.to_string();

    match student.read_by_email(&pool).await {
        Ok(_) => {
            match timestamp.parse::<i64>() {
                Ok(t) => {
                    let days = timestamps::difference_in_days(timestamps::get_unix_timestamp(), t);
                    log::debug!("{} {} {}", t, timestamps::get_unix_timestamp(), days);
                    if days == 0 {
                        // show the password reset text boxes
                        context.insert("email_hash", &email_hash);
                        render_template!(&tera, "auth/reset-password.html", &context)
                    } else {
                        context.insert("error", "link has expired");
                        render_template!(&tera, "auth/reset-password.html", &context)
                    }
                }
                Err(_) => {
                    context.insert("error", "invalid link: timestamp");
                    render_template!(&tera, "auth/reset-password.html", &context)
                }
            }
        }
        Err(_) => {
            context.insert("error", "invalid link: email");
            render_template!(&tera, "auth/reset-password.html", &context)
        }
    }
}

#[post("/reset-password")]
pub async fn reset_password_post(
    pool: web::Data<MySqlPool>,
    model: web::Json<ResetPasswordModel>,
) -> impl Responder {
    log::debug!("{:?}", model);
    // validate model
    if let Err(e) = model.validate() {
        bad_request!(e);
    }

    let mut student = Student::new();
    student.email_hash = model.email_hash.to_string();

    // get the student by email_hash then update its password
    match student.read_by_email(&pool).await {
        Ok(student_opt) => match student_opt {
            Some(mut student) => {
                let p_hash = hasher::cook_hash(&model.password).unwrap();
                log::debug!("passwords old: {:?} new: {}", student.pass_hash, p_hash);
                student.pass_hash = Some(p_hash);
                match student.update_pass_hash(&pool).await {
                    Ok(_) => ok_action!(ActionType::Redirect, "/login"),
                    Err(e) => {
                        server_error!("Failed to update password.", e)
                    }
                }
            }
            None => not_found!("Student not found"),
        },
        Err(e) => server_error!("Failed to retrieve student", e),
    }
}
