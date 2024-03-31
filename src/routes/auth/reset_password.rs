use actix_web::{ get, post, web, HttpResponse, Responder };
use serde::{ Deserialize, Serialize };
use sqlx::MySqlPool;
use tera::{ Context, Tera };

use crate::{ models::database::student::Student, utils::timestamps };

#[derive(Serialize, Deserialize, Debug)]
pub struct Base64ResetPasswordModel {
    pub q: String,
}

fn extract_values(s: &str) -> (Option<&str>, Option<&str>) {
    let mut e = None;
    let mut t = None;

    for param in s.split('&') {
        let mut parts = param.split('=');
        match (parts.next(), parts.next()) {
            (Some("e"), Some(value)) => {
                e = Some(value);
            }
            (Some("t"), Some(value)) => {
                t = Some(value);
            }
            _ => {}
        }
    }

    (e, t)
}

fn render_template(tera: &Tera, path: &str, context: &Context) -> HttpResponse {
    match tera.render("auth/reset-password.html", context) {
        Ok(body) => HttpResponse::Ok().content_type("text/html").body(body),
        Err(e) => {
            println!("Error rendering template: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[get("/reset-password")]
pub async fn reset_password_get(
    tera: web::Data<Tera>,
    pool: web::Data<MySqlPool>,
    query: web::Query<Base64ResetPasswordModel>
) -> impl Responder {
    // accept email address and verification code
    // verify them
    // if they match then offer allow them to enter and confirm a new password
    // if they don't match then redirect to /verify
    let mut context = Context::new();
    context.insert("title", &"Reset your password");

    let values = extract_values(&query.q);

    log::debug!("{:?}", values);

    // use e = email_hash, and t=timestamp to determine the validity of this link (T+1 day)

    let email_hash = values.0.unwrap();
    let timestamp = values.1.unwrap();
    // let timestamp = "1712071745";

    match Student::read_by_email_hash(&pool, &email_hash).await {
        Ok(_) => {
            match timestamp.parse::<i64>() {
                Ok(t) => {
                    let days = timestamps::difference_in_days(timestamps::get_unix_timestamp(), t);
                    log::debug!("{} {} {}", t, timestamps::get_unix_timestamp(), days);
                    if days == 0 {
                        // show the password reset text boxes
                        render_template(&tera, "auth/reset_password.html", &context)
                    } else {
                        context.insert("error", "link has expired");
                        render_template(&tera, "auth/reset_password.html", &context)
                    }
                }
                Err(_) => {
                    context.insert("error", "invalid link: timestamp");
                    render_template(&tera, "auth/reset_password.html", &context)
                }
            }
        }
        Err(_) => {
            context.insert("error", "invalid link: email");
            render_template(&tera, "auth/reset_password.html", &context)
        }
    }
}

#[post("/reset-password")]
pub async fn reset_password_post(
    tera: web::Data<Tera>,
    pool: web::Data<MySqlPool>
) -> impl Responder {
    let context = Context::new();
    render_template(&tera, "auth/reset_password.html", &context)
}
