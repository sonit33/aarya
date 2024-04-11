use aarya_utils::{
    json_ops::{self, read_json_file},
    openai_ops::completion_model::CompletionResponse,
};

#[test]
fn test_validate_json_file() {
    // Paths to schema and data files
    let schema_path = "../.schema/question-schema.json";
    let data_text = read_json_file("../.temp-data/co-2-ch-1/chatcmpl-9CgDIsesoHPf22DGCksGCamth2xpb.txt").unwrap();
    let model: CompletionResponse = serde_json::from_value(data_text).unwrap();

    match json_ops::validate_json_text(&schema_path, model.choices[0].message.content.as_str()) {
        Ok(_) => {
            println!("schema valid");
        }
        Err(e) => panic!("{:?}", e),
    }
}
