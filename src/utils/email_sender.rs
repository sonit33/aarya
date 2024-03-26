use std::error::Error;

// Include necessary crates
use lettre::{AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor};
use lettre::transport::smtp::authentication::Credentials;

// Define a struct for the EmailSender
pub struct EmailSender {
	server: String,
	port: u16,
	username: String,
	// Placeholder, might not be needed for MailHog but included for completeness
	password: String, // Placeholder, might not be needed for MailHog but included for completeness
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
	pub async fn send_email(&self, to: &str, subject: &str, body: &str) -> Result<(), Box<dyn Error>> {
		// Construct the message
		let email = Message::builder()
			.from(self.username.parse()?)
			.to(to.parse()?)
			.subject(subject)
			.body(String::from(body))?;

		// Setup the SMTP transport using MailHog configuration
		let creds = Credentials::new(self.username.clone(), self.password.clone());

		let mailer = AsyncSmtpTransport::<Tokio1Executor>::relay(&self.server)?
			.port(self.port)
			.credentials(creds)
			.build();

		// Send the email
		mailer.send(email).await?;

		Ok(())
	}
}
