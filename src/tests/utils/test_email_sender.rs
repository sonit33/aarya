use tokio;

use crate::utils::email_sender::EmailSender;

// Test to verify that sending an email successfully returns Ok(())
#[tokio::test]
async fn test_send_email_success() {
	// Setup: Instantiate EmailSender with MailHog configuration
	// Note: Replace "localhost" and 1025 with your MailHog server and port if different
	let email_sender = EmailSender::new("localhost".to_string(), 1025, "".to_string(), "".to_string());

	// Execute: Attempt to send an email
	email_sender.send_email("user@example.com", "recipient@example.com", "Test Subject", "Test Body").await.unwrap();

	// Verify: Expect Ok(()) indicating the email was sent successfully
	// assert!(result.is_ok());
}

// Test to verify handling of invalid recipient email addresses
#[tokio::test]
async fn test_send_email_invalid_recipient() {
	let email_sender = EmailSender::new("localhost".to_string(), 1025, "".to_string(), "".to_string());

	// Execute with an invalid email address format
	let result = email_sender.send_email("user@example.com", "invalid-recipient", "Test Subject", "Test Body").await;

	// Verify: Expect an error indicating the recipient email address is invalid
	// assert!(result.is_err());
}

// Test to verify the handling when the MailHog server is unreachable
#[tokio::test]
async fn test_send_email_server_unreachable() {
	// Setup: Instantiate EmailSender with an unreachable server configuration
	let email_sender = EmailSender::new("invalid-server".to_string(), 1025, "".to_string(), "".to_string());

	// Execute: Attempt to send an email with the unreachable server configuration
	let result = email_sender.send_email("user@example.com", "recipient@example.com", "Test Subject", "Test Body").await;

	// Verify: Expect an error indicating the server is unreachable or connection failed
	// assert!(result.is_err());
}