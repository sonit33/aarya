use actix_web::{get, web, HttpResponse, Responder};
use handlebars::Handlebars;
use serde_json::json;

#[get("/")]
pub async fn home(handlebars: web::Data<Handlebars<'_>>) -> impl Responder {
    // Render the index template using Handlebars
    match handlebars.render("index", &json!({"title": "Aarya welcomes you!"})) {
        Ok(body) => HttpResponse::Ok().body(body),
        Err(e) => {
            println!("Error rendering index template: {:?}", e);

            HttpResponse::InternalServerError().finish()
        }
    }
}
