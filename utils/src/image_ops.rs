use base64::prelude::*;
use std::fs::File;
use std::io::Read;

use crate::util_error::AaryaUtilError;

pub fn encode_to_base64(image_path: &str) -> Result<String, AaryaUtilError> {
    match File::open(image_path) {
        Ok(mut file) => {
            // Read the file's contents into a vector
            let mut buffer = Vec::new();
            match file.read_to_end(&mut buffer) {
                Ok(_) => {
                    let encoded_image = BASE64_STANDARD.encode(&buffer);
                    Ok(encoded_image)
                }
                Err(e) => return Err(AaryaUtilError::FileReadError(format!("{}", e), image_path.to_string())),
            }
        }
        Err(e) => Err(AaryaUtilError::FileOpenError(format!("{}", e), image_path.to_string())),
    }
}
