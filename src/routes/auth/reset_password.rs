use actix_web::{ get, post, web, HttpResponse, Responder };
use tera::{ Context, Tera };

// #[post("/reset-password")]
// pub async fn reset_password_post() -> impl Responder {
//     // accept email address, verification code, and new passwords
//     // verify email address and code
//     // if verified then change the password
//     // if not verified then redirect to /verify
// }

#[get("/reset-password")]
pub async fn reset_password_get(tera: web::Data<Tera>) -> impl Responder {
    // accept email address and verification code
    // verify them
    // if they match then offer allow them to enter and confirm a new password
    // if they don't match then redirect to /verify
    let mut context = Context::new();
    context.insert("title", &"Reset your password");

    match tera.render("auth/reset-password.html", &context) {
        Ok(body) => HttpResponse::Ok().content_type("text/html").body(body),
        Err(e) => {
            println!("Error rendering template: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}
