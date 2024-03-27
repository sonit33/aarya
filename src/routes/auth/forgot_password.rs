use actix_web::{ get, post, web, HttpResponse, Responder };
use tera::{ Context, Tera };

// #[post("/forgot-password")]
// pub async fn forgot_password_post() -> impl Responder {
//     // accept email address
//     // verify email address exists in the database
//     // if it exists then send a new verification code to the email address
//     // redirect to /verify
//     // if it doesn't exist then ask to signup
// }

#[get("/forgot-password")]
pub async fn forgot_password_get(tera: web::Data<Tera>) -> impl Responder {
    let mut context = Context::new();
    context.insert("title", &"Forgot password?");

    match tera.render("auth/forgot-password.html", &context) {
        Ok(body) => HttpResponse::Ok().content_type("text/html").body(body),
        Err(e) => {
            println!("Error rendering template: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}
