use jsonschema::{Draft, JSONSchema};
use serde_json::{from_reader, Value};
use std::fs::File;
use std::io::BufReader;

// Function to read JSON file
fn read_json_file(file_path: &str) -> Value {
    let file = File::open(file_path).expect("Failed to open file");
    let reader = BufReader::new(file);
    from_reader(reader).expect("Failed to parse JSON")
}

#[test]
fn test_validate_json_file() {
    // Paths to schema and data files
    let schema_path = ".schema/question-schema.json";
    let data_path = ".temp-data/dump-040724.json";

    // Read schema and data files
    let schema = read_json_file(schema_path);
    let data = read_json_file(data_path);

    let compiled = JSONSchema::options()
        .with_draft(Draft::Draft7)
        .compile(&schema)
        .expect("A valid schema");

    let result = compiled.validate(&data);
    assert!(result.is_ok());
    // if let Err(errors) = result {
    //     for error in errors {
    //         println!("Validation error: {}", error);
    //         println!("Instance path: {}", error.instance_path);
    //     }
    // }
}
