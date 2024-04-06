use crate::errors::email_error::EmailError;

// Define a struct for the EmailSender
#[derive(Debug, Clone)]
pub struct EmailSender {}

impl EmailSender {
    // Asynchronous method to send email
    // Parameters include recipient, subject, and body of the email
    pub async fn send_email(
        &self,
        from: &str,
        to: &str,
        subject: &str,
        body: &str
    ) -> Result<(), EmailError> {
        log::debug!("Mock email From: {} To: {} Subject: {}, Body: {}", from, to, subject, body);

        Ok(())
    }
}
