use std::path::Path;

use aarya_utils::{
    file_ops,
    json_ops::{self, json_to_vec, JsonOpsResult},
};
use models::{questions::QuestionEntity, result_types::EntityResult};
use sqlx::MySqlPool;

use super::batchgener::ManifestModel;

pub async fn run_batch_uploads(
    schema_file: &Path,
    directory: &Path,
    pool: &MySqlPool,
) {
    if !directory.is_dir() {
        println!("Directory path is required and does not exist");
        return;
    }

    // read the manifest file
    println!("Reading manifest file from directory: {:?}", directory.to_str().unwrap());
    let contents = match file_ops::read_file_contents(directory.join("manifest.json").to_str().unwrap()) {
        file_ops::FileOpsResult::Success(r) => r,
        file_ops::FileOpsResult::Error(_) => {
            println!("Failed to read manifest file");
            return;
        }
    };

    // parse the manifest file
    println!("Parsing manifest file");
    let models: Vec<ManifestModel> = match serde_json::from_str(contents.as_str()) {
        Ok(m) => m,
        Err(e) => {
            println!("Failed to parse json: {:?}", e);
            return;
        }
    };

    // iterate the parsed manifest models
    println!("Iterating manifest models");
    for model in models {
        // validate
        println!("Validating file: {}", &model.file_path.as_str());
        match json_ops::validate_json_file(schema_file.to_str().unwrap(), &model.file_path) {
            JsonOpsResult::Success(_) => println!("File validated successfully: {}", &model.file_path),
            JsonOpsResult::Error(e) => {
                println!("Failed to validate file: {:?}", e);
                return;
            }
        }

        // load the questions
        println!("Loading questions from file: {}", &model.file_path.as_str());
        let questions = match json_to_vec::<QuestionEntity>(&model.file_path) {
            JsonOpsResult::Success(q) => q,
            JsonOpsResult::Error(e) => {
                println!("Failed to parse json: {:?}", e);
                return;
            }
        };

        // save the questions
        println!("Saving {} question(s)", questions.len());
        for question in questions {
            println!("Saving question Id: {:?}", question.question_id.unwrap());
            match question.create(pool).await {
                EntityResult::Success(_) => println!("Question: {:?} saved successfully", question.question_id),
                EntityResult::Error(e) => {
                    println!("Failed to save question: {:?}", e);
                    return;
                }
            }
        }
    }
}
