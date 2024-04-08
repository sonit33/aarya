use jsonschema::{Draft, JSONSchema};
use serde_json::{from_reader, Value};
use std::error::Error;
use std::fs::File;
use std::io::BufReader;

// Function to read JSON file
fn read_json_file(file_path: &str) -> Value {
    let file = File::open(file_path).expect("Failed to open file");
    let reader = BufReader::new(file);
    from_reader(reader).expect("Failed to parse JSON")
}

pub fn validate_json_file(schema_path: &str, data_path: &str) -> Result<bool, Box<dyn Error>> {
    let schema = read_json_file(schema_path);
    let data = read_json_file(data_path);

    let compiled = JSONSchema::options()
        .with_draft(Draft::Draft7)
        .compile(&schema)
        .expect("A valid schema");

    let result = compiled.validate(&data);
    match result.is_ok() {
        true => Ok(true),
        false => Err("Validation failed".into()), // Return a specific error message
    }
}
