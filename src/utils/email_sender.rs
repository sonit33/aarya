use crate::errors::email_error::EmailError;

// Define a struct for the EmailSender
#[derive(Debug, Clone)]
pub struct EmailSender {
	server: String,
	port: u16,
	username: String,
	password: String,
}

impl EmailSender {
	// Constructor to initialize EmailSender with server details
	pub fn new(server: String, port: u16, username: String, password: String) -> Self {
		Self {
			server,
			port,
			username,
			password,
		}
	}

	// Asynchronous method to send email
	// Parameters include recipient, subject, and body of the email
	pub async fn send_email(&self, from: &str, to: &str, subject: &str, body: &str) -> Result<(), EmailError> {
		println!("Mock email From: {} To: {} Subject: {}, Body: {}", from, to, subject, body);

		Ok(())
	}
}
