use std::error::Error;
use std::fmt::Display;

pub mod model_error {
    use std::error::Error;
    use std::fmt::{Display, Formatter};

    #[derive(Debug)]
    pub struct GeneralModelError {
        pub reason: String,
    }

    impl Display for GeneralModelError {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(f, "User model failed due to {}", self.reason)
        }
    }

    impl Error for GeneralModelError {}
}