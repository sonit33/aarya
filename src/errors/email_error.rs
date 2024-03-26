use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub struct EmailError {
	pub reason: String,
}

impl Display for EmailError {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		write!(f, "Email failed due to {}", self.reason)
	}
}

impl Error for EmailError {}