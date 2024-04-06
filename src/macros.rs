#[macro_export]
macro_rules! string_response {
    ($action:expr, $arg:expr) => {
        DefaultResponseModel::<String> {
            action_type: $action,
            arg: $arg.to_string(),
        }
    };
}

#[macro_export]
macro_rules! server_error {
    ($arg1:expr, $arg2:ident) => {
        {
        println!("{:?}", $arg1);
        println!("{:?}", $arg2);

        HttpResponse::InternalServerError().json(
            DefaultResponseModel::<String> {
                action_type: ActionType::HandleError,
                arg: format!("{}: {}", $arg1, $arg2)
        })
        }
    };
}

#[macro_export]
macro_rules! all_ok {
    ($arg:expr) => {
        HttpResponse::Ok().json(
            DefaultResponseModel::<String> {
                action_type: ActionType::Inform,
                arg: $arg.to_string()
        })
    };
}

#[macro_export]
macro_rules! ok_action {
    ($arg1:expr, $arg2:expr) => {
        HttpResponse::Ok().json(
            DefaultResponseModel::<String> {
                action_type: $arg1,
                arg: $arg2.to_string()
        })
    };
}

#[macro_export]
macro_rules! bad_request {
    ($arg:expr) => {
        HttpResponse::BadRequest().json(
            DefaultResponseModel::<String> {
                action_type: ActionType::HandleError,
                arg: $arg.to_string()
        })
    };
}

#[macro_export]
macro_rules! forbidden {
    ($arg:expr) => {
        HttpResponse::Forbidden().json(
            DefaultResponseModel::<String> {
                action_type: ActionType::HandleError,
                arg: $arg.to_string()
        })
    };
}

#[macro_export]
macro_rules! unauthorized {
    ($arg:expr) => {
        HttpResponse::Unauthorized().json(
            DefaultResponseModel::<String> {
                action_type: ActionType::HandleError,
                arg: $arg.to_string()
        })
    };
}

#[macro_export]
macro_rules! not_found {
    ($arg:expr) => {
        HttpResponse::NotFound().json(
            DefaultResponseModel::<String> {
                action_type: ActionType::HandleError,
                arg: $arg.to_string()
        })
    };
}

#[macro_export]
macro_rules! render_template {
    ($tera:expr, $template:expr, $context:expr) => {
        match $tera.render($template, $context) {
            Ok(body) => HttpResponse::Ok().content_type("text/html").body(body),
            Err(e) => {
                println!("Error rendering template: {}", e);
                HttpResponse::InternalServerError().finish()
            }
        }
    };
}
