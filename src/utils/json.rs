use jsonschema::JSONSchema;
use serde_json::Value;
use std::fs::File;
use std::io::BufReader;

fn validate_json_file(file_path: &str, schema: &str) -> bool {
    // Read the JSON file
    let file = File::open(file_path).expect("Failed to open file");
    let reader = BufReader::new(file);
    let json_data: Value = serde_json::from_reader(reader).expect("Failed to parse JSON");

    // Parse the JSON schema
    let schema_data: Value = serde_json::from_str(schema).expect("Failed to parse schema");

    // Create a JSONSchema from the parsed schema
    let compile = JSONSchema::compile(&schema_data);
    let compiled_schema = compile.expect("Failed to compile schema");

    // Validate the JSON data against the schema
    let validation_result = compiled_schema.validate(&json_data);

    // Return true if validation succeeded, false otherwise
    validation_result.is_ok()
}
