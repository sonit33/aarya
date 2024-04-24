use base64::prelude::*;
use std::fs::File;
use std::io::Read;

pub enum ImageOpsErrorTypes {
    FileOpenError(String, String),
    FileReadError(String, String),
}
pub enum ImageOpsResult {
    Success(String),
    Error(ImageOpsErrorTypes),
}

pub fn encode_to_base64(image_path: &str) -> ImageOpsResult {
    match File::open(image_path) {
        Ok(mut file) => {
            // Read the file's contents into a vector
            let mut buffer = Vec::new();
            match file.read_to_end(&mut buffer) {
                Ok(_) => {
                    let encoded_image = BASE64_STANDARD.encode(&buffer);
                    ImageOpsResult::Success(encoded_image)
                }
                Err(e) => ImageOpsResult::Error(ImageOpsErrorTypes::FileReadError(format!("{}", e), image_path.to_string())),
            }
        }
        Err(e) => ImageOpsResult::Error(ImageOpsErrorTypes::FileOpenError(format!("{}", e), image_path.to_string())),
    }
}
