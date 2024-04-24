macro_rules! render_template {
    ($handlebars:expr, $template_name:expr, $handlebars_context:expr) => {
        match $handlebars.render($template_name, &json!($handlebars_context)) {
            Ok(body) => HttpResponse::Ok().body(body),
            Err(e) => {
                println!("Error rendering {} template: {:?}", $template_name, e);
                HttpResponse::InternalServerError().finish()
            }
        }
    };
}
