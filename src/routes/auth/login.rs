use actix_web::{ web, HttpResponse, Responder, post, get };
use tera::Context;
use tera::Tera;
use validator::Validate;
use sqlx::MySqlPool;

use crate::models::auth::login::LoginModel;
use crate::models::database::student::Student;
use crate::models::default_response::ResponseAction;
use crate::models::default_response::ActionType;
use crate::models::default_response::DefaultResponseModel;
use crate::utils::hasher;

#[get("/login")]
async fn login_get(tera: web::Data<Tera>) -> impl Responder {
    let mut context = Context::new();
    context.insert("title", &"Login to Aarya");

    match tera.render("auth_login.html", &context) {
        Ok(body) => HttpResponse::Ok().content_type("text/html").body(body),
        Err(e) => {
            println!("Error rendering template: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[post("/login")]
async fn login_post(pool: web::Data<MySqlPool>, model: web::Json<LoginModel>) -> impl Responder {
    // Validate the LoginModel
    if let Err(e) = model.validate() {
        return HttpResponse::BadRequest().json(DefaultResponseModel::<()> {
            message: format!("Validation error: {}", e),
            payload: (),
            action: ResponseAction {
                action_type: ActionType::Resolve,
                arg: "".to_string(),
            },
        });
    }

    let login = model.clone();

    // Query the database for a user with the given email
    let user = match Student::read_by_email(&pool, &login.email).await {
        Ok(user) =>
            match user {
                Some(user) => user,
                None => {
                    return HttpResponse::BadRequest().json(DefaultResponseModel::<()> {
                        message: "User not found.".to_string(),
                        payload: (),
                        action: ResponseAction {
                            action_type: ActionType::Resolve,
                            arg: "".to_string(),
                        },
                    });
                }
            }
        Err(e) => {
            println!("{:?}", e);
            return HttpResponse::InternalServerError().json(DefaultResponseModel::<()> {
                message: format!("Database error: {}", e),
                payload: (),
                action: ResponseAction {
                    action_type: ActionType::Resolve,
                    arg: "".to_string(),
                },
            });
        }
    };

    // Verify the supplied password matches the one stored in the database
    if !hasher::verify(&model.password, &user.password) {
        return HttpResponse::Unauthorized().json(DefaultResponseModel::<()> {
            message: "Invalid credentials.".to_string(),
            payload: (),
            action: ResponseAction {
                action_type: ActionType::Resolve,
                arg: "".to_string(),
            },
        });
    }

    // Check if the user's account is active and email is verified
    if !user.email_verified || !user.account_active {
        return HttpResponse::Forbidden().json(DefaultResponseModel::<()> {
            message: "Account not active or email not verified.".to_string(),
            payload: (),
            action: ResponseAction {
                action_type: ActionType::Resolve,
                arg: "".to_string(),
            },
        });
    }

    // Success: user authenticated
    HttpResponse::Ok().json(DefaultResponseModel::<String> {
        message: "Login successful.".to_string(),
        payload: "/home".to_string(),
        action: ResponseAction {
            action_type: ActionType::Redirect,
            arg: "/home".to_string(),
        },
    })
}
