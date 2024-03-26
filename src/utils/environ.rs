use std::env;

#[derive(Debug, Clone)]
pub struct Environ {
	pub db_connection_string: String,
	pub db_name: String,
	pub email_server: String,
	pub email_port: String,
	pub email_username: String,
	pub email_password: String
}

impl Default for Environ {
	fn default() -> Self {
		let db_cs = env::var("DB_CS").expect("Missing DB connection string");
		let db_name = env::var("DB_NAME").expect("Missing DB database name");
		let email_server = env::var("EMAIL_SERVER").expect("Missing Email server name");
		let email_port = env::var("EMAIL_PORT").expect("Missing Email server name");
		let email_username = env::var("EMAIL_USERNAME").expect("Missing Email server name");
		let email_password = env::var("EMAIL_PASSWORD").expect("Missing Email server name");
		Environ { db_connection_string: db_cs, db_name, email_server, email_port, email_username, email_password }
	}
}