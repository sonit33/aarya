use std::{
    fs::File,
    io::{Read, Write},
};

use crate::util_error::AaryaUtilError;

// read file contents
pub fn read_file_contents(path: &str) -> Result<String, AaryaUtilError> {
    match File::open(path) {
        Ok(mut file) => {
            let mut contents = String::new();
            match file.read_to_string(&mut contents) {
                Ok(_) => Ok(contents),
                Err(e) => Err(AaryaUtilError::FileReadError(format!("{}", e), path.to_string())),
            }
        }
        Err(e) => Err(AaryaUtilError::FileOpenError(format!("{}", e), path.to_string())),
    }
}

// write to file
pub fn write_to_file(path: &str, contents: &str) -> Result<(), AaryaUtilError> {
    match File::create(path) {
        Ok(mut file) => match file.write_all(contents.as_bytes()) {
            Ok(_) => Ok(()),
            Err(e) => Err(AaryaUtilError::FileWriteError(format!("{}", e), path.to_string())),
        },
        Err(e) => Err(AaryaUtilError::FileCreateError(format!("{}", e), path.to_string())),
    }
}
