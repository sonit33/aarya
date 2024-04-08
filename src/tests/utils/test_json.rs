use crate::utils::json;

#[test]
fn test_validate_json_file() {
    // Paths to schema and data files
    let schema_path = ".schema/question-schema.json";
    let data_path = ".temp-data/co2-ch2-040724.json";

    let result = json::validate_json_file(&schema_path, &data_path);
    assert!(result.is_ok());
}
