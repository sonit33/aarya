use std::{fs::File, io::BufReader};

use jsonschema::{Draft, JSONSchema};
use serde::de::DeserializeOwned;
use serde::Serialize;
use serde_json::{from_reader, Value};

#[derive(Debug)]
pub enum JsonErrorTypes {
    FileOpenError(String, String),
    FileReadError(String, String),
    SchemaCompilationError(String, String),
    ValidationError(String),
    JsonParseError(String, String),
}

#[derive(Debug)]
pub enum JsonOpsResult<T> {
    Success(T),
    Error(JsonErrorTypes),
}

// Function to read JSON file
pub fn read_json_file(file_path: &str) -> JsonOpsResult<Value> {
    match File::open(file_path) {
        Ok(f) => {
            let reader = BufReader::new(f);
            match from_reader(reader) {
                Ok(contents) => JsonOpsResult::Success(contents),
                Err(e) => JsonOpsResult::Error(JsonErrorTypes::FileReadError(
                    format!("{}", e),
                    file_path.to_string(),
                )),
            }
        }
        Err(e) => JsonOpsResult::Error(JsonErrorTypes::FileOpenError(
            format!("{}", e),
            file_path.to_string(),
        )),
    }
}

pub fn validate_json_file(schema_path: &str, data_path: &str) -> JsonOpsResult<bool> {
    let schema = match read_json_file(schema_path) {
        JsonOpsResult::Success(schema) => schema,
        JsonOpsResult::Error(e) => {
            return JsonOpsResult::Error(e);
        }
    };

    let data = match read_json_file(data_path) {
        JsonOpsResult::Success(data) => data,
        JsonOpsResult::Error(e) => {
            return JsonOpsResult::Error(e);
        }
    };

    match JSONSchema::options()
        .with_draft(Draft::Draft7)
        .compile(&schema)
    {
        Ok(c) => match c.validate(&data) {
            Ok(_) => JsonOpsResult::Success(true),
            Err(e) => {
                for error in e {
                    println!("Validation error: {}", error);
                }
                JsonOpsResult::Error(JsonErrorTypes::ValidationError(
                    "Validation failed".to_string(),
                ))
            }
        },
        Err(e) => JsonOpsResult::Error(JsonErrorTypes::SchemaCompilationError(
            format!("{}", e),
            schema_path.to_string(),
        )),
    }
}

pub fn validate_json_text(schema_path: &str, json_text: &str) -> JsonOpsResult<bool> {
    let schema = match read_json_file(schema_path) {
        JsonOpsResult::Success(schema) => schema,
        JsonOpsResult::Error(e) => {
            return JsonOpsResult::Error(e);
        }
    };

    let data: Value = match serde_json::from_str(json_text) {
        Ok(data) => data,
        Err(e) => {
            return JsonOpsResult::Error(JsonErrorTypes::JsonParseError(
                format!("{}", e),
                json_text.to_string(),
            ));
        }
    };

    match JSONSchema::options()
        .with_draft(Draft::Draft7)
        .compile(&schema)
    {
        Ok(c) => {
            let result = c.validate(&data);
            match result.is_ok() {
                true => JsonOpsResult::Success(true),
                false => {
                    let mut errors: Vec<String> = vec![];
                    for error in result.err().unwrap() {
                        errors.push(format!("{:?}", error));
                    }
                    JsonOpsResult::Error(JsonErrorTypes::ValidationError(format!("{:?}", errors)))
                }
            }
        }
        Err(e) => JsonOpsResult::Error(JsonErrorTypes::SchemaCompilationError(
            format!("{}", e),
            schema_path.to_string(),
        )),
    }
}

pub fn json_to_vec<T>(file_path: &str) -> JsonOpsResult<Vec<T>>
where
    T: Serialize + DeserializeOwned,
{
    // Open the file in read-only mode
    let file = match File::open(file_path) {
        Ok(f) => f,
        Err(e) => {
            return JsonOpsResult::Error(JsonErrorTypes::FileOpenError(
                format!("{}", e),
                file_path.to_string(),
            ));
        }
    };

    // Create a buffered reader to read the file
    let reader = BufReader::new(file);
    // Deserialize the JSON data into a vector of YourStruct
    let data: Vec<T> = match serde_json::from_reader(reader) {
        Ok(d) => d,
        Err(e) => {
            return JsonOpsResult::Error(JsonErrorTypes::JsonParseError(
                format!("{}", e),
                file_path.to_string(),
            ));
        }
    };

    JsonOpsResult::Success(data)
}
