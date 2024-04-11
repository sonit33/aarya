use aarya_macros::forbidden;
use aarya_macros::render_template;
use aarya_macros::unauthorized;
use aarya_macros::{bad_request, not_found, ok_action, server_error};
use aarya_models::auth::login::LoginModel;
use aarya_models::database::student::Student;
use aarya_models::default_response::ActionType;
use aarya_models::default_response::DefaultResponseModel;
use aarya_utils::hasher;
use actix_web::{get, post, web, HttpResponse, Responder};
use sqlx::MySqlPool;
use tera::{Context, Tera};
use validator::Validate;

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

    let mut student = Student::new();
    student.email_address = login.email_address;

    // Query the database for a user with the given email
    match student.read_by_email(&pool).await {
        Ok(user) => match user {
            Some(user) => {
                // Verify the supplied password matches the one stored in the database
                if !hasher::verify(&model.password, &user.pass_hash.unwrap()) {
                    return unauthorized!("Invalid credentials.");
                }

                // Check if the user's account is active and email is verified
                if !user.email_verified || !user.account_active {
                    return forbidden!("Your account is not verified. Follow this link to <a href='/activate-account'>activate your account</a>");
                }
            }
            None => {
                return not_found!("We did not find any user with that email address. Please check your email and try again.");
            }
        },
        Err(e) => {
            return server_error!("Failed to retrieve user information", e);
        }
    }

    ok_action!(ActionType::Redirect, "/home")
}
