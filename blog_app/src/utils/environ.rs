use std::env;

use dotenv::from_filename;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Environ {
    pub db_connection_string: String,
    pub db_name: String,
    pub allowed_origin: String,
    pub web_app_port: u16,
}

impl Environ {
    pub fn init() {
        let env_file = if cfg!(debug_assertions) {
            ".env.dev"
        } else {
            ".env.prod"
        };

        from_filename(env_file).ok();
    }
}

impl Default for Environ {
    fn default() -> Self {
        Environ::init();
        let db_cs = env::var("DB_CONNECTION_STRING").expect("Missing DB connection string");
        let db_name = env::var("DB_NAME").expect("Missing DB name");
        let allowed_origin = env::var("ALLOWED_ORIGIN").expect("Missing ALLOWED_ORIGIN");
        let web_app_port = env::var("WEB_APP_PORT")
            .expect("Missing WEB_APP_PORT")
            .parse::<u16>()
            .expect("WEB_APP_PORT must be a number");
        Environ {
            db_connection_string: db_cs,
            db_name,
            allowed_origin,
            web_app_port,
        }
    }
}
