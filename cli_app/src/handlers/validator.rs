use std::path::Path;

use aarya_utils::{
    file_ops::file_exists,
    json_ops::{self, JsonOpsResult},
};

pub async fn run_validate(schema_file: &Path, data_file: &Path) {
    let schema_file = schema_file.to_str().unwrap();
    let data_file = data_file.to_str().unwrap();

    if !file_exists(schema_file) {
        println!("Schema file does not exist");
        return;
    }

    if !file_exists(data_file) {
        println!("Data file does not exist");
        return;
    }

    println!("Validating schema file: {:?} and data file: {:?}", schema_file, data_file);

    match json_ops::validate_json_file(schema_file, data_file) {
        JsonOpsResult::Success(_) => {
            println!("Validation successful");
        }
        JsonOpsResult::Error(e) => {
            println!("Validation failed: {:?}", e);
        }
    }
}
