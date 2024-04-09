use crate::util_error::AaryaUtilError;
use jsonschema::{Draft, JSONSchema};
use serde::de::DeserializeOwned;
use serde::Serialize;
use serde_json::{from_reader, Value};
use std::{error::Error, fs::File, io::BufReader};

// Function to read JSON file
fn read_json_file(file_path: &str) -> Result<Value, AaryaUtilError> {
    match File::open(file_path) {
        Ok(f) => {
            let reader = BufReader::new(f);
            match from_reader(reader) {
                Ok(contents) => Ok(contents),
                Err(e) => Err(AaryaUtilError::FileReadError(
                    format!("{}", e),
                    file_path.to_string(),
                )),
            }
        }
        Err(e) => Err(AaryaUtilError::FileOpenError(
            format!("{}", e),
            file_path.to_string(),
        )),
    }
}

pub fn validate_json_file(schema_path: &str, data_path: &str) -> Result<bool, AaryaUtilError> {
    let schema = read_json_file(schema_path)?;
    let data = read_json_file(data_path)?;

    match JSONSchema::options()
        .with_draft(Draft::Draft7)
        .compile(&schema)
    {
        Ok(c) => {
            let result = c.validate(&data);
            match result.is_ok() {
                true => Ok(true),
                false => Err(AaryaUtilError::ValidationError(data_path.to_string())), // Return a specific error message
            }
        }
        Err(e) => Err(AaryaUtilError::SchemaCompilationError(
            format!("{}", e),
            schema_path.to_string(),
        )),
    }
}

pub fn json_to_vec<T>(file_path: &str) -> Result<Vec<T>, Box<dyn Error>>
where
    T: Serialize + DeserializeOwned,
{
    // Open the file in read-only mode
    let file = File::open(file_path)?;
    // Create a buffered reader to read the file
    let reader = BufReader::new(file);
    // Deserialize the JSON data into a vector of YourStruct
    let data: Vec<T> = serde_json::from_reader(reader)?;

    Ok(data)
}
