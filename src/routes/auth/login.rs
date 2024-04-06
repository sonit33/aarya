use actix_web::{ get, HttpResponse, post, Responder, web };
use sqlx::MySqlPool;
use tera::Context;
use tera::Tera;
use validator::Validate;

use crate::bad_request;
use crate::forbidden;
use crate::models::auth::login::LoginModel;
use crate::models::database::student::Student;
use crate::models::default_response::ActionType;
use crate::models::default_response::DefaultResponseModel;
use crate::not_found;
use crate::ok_action;
use crate::render_template;
use crate::server_error;
use crate::unauthorized;
use crate::utils::hasher;

#[get("/login")]
async fn login_get(tera: web::Data<Tera>) -> impl Responder {
    let mut context = Context::new();
    context.insert("title", &"Login to Aarya");

    render_template!(&tera, "auth/login.html", &context)
}

#[post("/login")]
async fn login_post(pool: web::Data<MySqlPool>, model: web::Json<LoginModel>) -> impl Responder {
    // Validate the LoginModel
    if let Err(e) = model.validate() {
        return bad_request!(format!("Validation error: [{e}]"));
    }

    let login = model.clone();

    // Query the database for a user with the given email
    match Student::read_by_email(&pool, &login.email_address).await {
        Ok(user) =>
            match user {
                Some(user) => {
                    // Verify the supplied password matches the one stored in the database
                    if !hasher::verify(&model.password, &user.password) {
                        return unauthorized!("Invalid credentials.");
                    }

                    // Check if the user's account is active and email is verified
                    if !user.email_verified || !user.account_active {
                        return forbidden!(
                            "Your account is not verified. Follow this link to <a href='/activate-account'>activate your account</a>"
                        );
                    }
                }
                None => {
                    return not_found!(
                        "We did not find any user with that email address. Please check your email and try again."
                    );
                }
            }
        Err(e) => {
            return server_error!(format!("Failed to retrieve user information: [{e}]"));
        }
    }

    ok_action!(ActionType::Redirect, "/home")
}
