use bcrypt::{ hash, DEFAULT_COST };

pub fn generate_password_hash(password: &str) -> Result<String, bcrypt::BcryptError> {
    hash(password, DEFAULT_COST)
}

pub fn verify(text: &str, hashed: &str) -> bool {
    bcrypt::verify(text, hashed).unwrap()
}
