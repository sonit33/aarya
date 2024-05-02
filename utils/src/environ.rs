use std::env;

use dotenv::from_filename;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Environ {
    pub db_connection_string: String,
    pub db_name: String,
    pub email_server: String,
    pub email_port: String,
    pub email_username: String,
    pub email_password: String,
    pub openai_key: String,
    pub allowed_origin: String,
    pub web_app_port: u16,
    pub blog_app_port: u16,
}

impl Environ {
    pub fn init() {
        let env_file = if cfg!(debug_assertions) { ".env.dev" } else { ".env.prod" };

        from_filename(env_file).ok();
    }
}

impl Default for Environ {
    fn default() -> Self {
        Environ::init();
        let db_cs = env::var("DB_CONNECTION_STRING").expect("Missing DB connection string");
        let db_name = env::var("DB_NAME").expect("Missing DB name");
        let email_server = env::var("EMAIL_SERVER").expect("Missing Email server name");
        let email_port = env::var("EMAIL_PORT").expect("Missing Email server name");
        let email_username = env::var("EMAIL_USERNAME").expect("Missing Email server name");
        let email_password = env::var("EMAIL_PASSWORD").expect("Missing Email server name");
        let openai_key = env::var("OPENAI_KEY").expect("Missing OpenAI key");
        let allowed_origin = env::var("ALLOWED_ORIGIN").expect("Missing ALLOWED_ORIGIN");
        let web_app_port = env::var("WEB_APP_PORT").expect("Missing WEB_APP_PORT").parse::<u16>().expect("WEB_APP_PORT must be a number");
        let blog_app_port = env::var("BLOG_APP_PORT").expect("Missing BLOG_APP_PORT").parse::<u16>().expect("BLOG_APP_PORT must be a number");
        Environ {
            db_connection_string: db_cs,
            db_name,
            email_server,
            email_port,
            email_username,
            email_password,
            openai_key,
            allowed_origin,
            web_app_port,
            blog_app_port,
        }
    }
}
