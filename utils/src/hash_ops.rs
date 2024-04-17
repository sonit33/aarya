use bcrypt::{ hash, DEFAULT_COST };
use crc32fast::Hasher;
use hex::encode as hex_encode;
use sha2::{ Digest, Sha256 };

pub fn string_hasher(input: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(input);
    let result = hasher.finalize();
    hex_encode(result)
}

pub fn cook_hash(password: &str) -> Result<String, bcrypt::BcryptError> {
    hash(password, DEFAULT_COST)
}

pub fn verify(text: &str, hashed: &str) -> bool {
    bcrypt::verify(text, hashed).unwrap()
}

pub fn fast_hash(input: &str) -> String {
    let mut hasher = Hasher::new();
    hasher.update(input.as_bytes());
    let checksum = hasher.finalize();
    format!("{:08x}", checksum)
}
