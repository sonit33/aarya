use std::env;

use dotenv::from_filename;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Environ {
    pub db_connection_string: String,
    pub email_server: String,
    pub email_port: String,
    pub email_username: String,
    pub email_password: String,
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
        let email_server = env::var("EMAIL_SERVER").expect("Missing Email server name");
        let email_port = env::var("EMAIL_PORT").expect("Missing Email server name");
        let email_username = env::var("EMAIL_USERNAME").expect("Missing Email server name");
        let email_password = env::var("EMAIL_PASSWORD").expect("Missing Email server name");
        Environ {
            db_connection_string: db_cs,
            email_server,
            email_port,
            email_username,
            email_password,
        }
    }
}
