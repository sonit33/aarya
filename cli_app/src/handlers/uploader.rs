use std::path::Path;

use aarya_utils::file_ops::{file_exists, read_file_contents, FileOpsResult};
use models::{questions::QuestionEntity, result_types::EntityResult};
use sqlx::MySqlPool;

pub async fn upload(course_id: u32, chapter_id: u32, topic_id: u32, data_file: &Path, pool: &MySqlPool) {
    let data_file = data_file.to_str().unwrap();
    if !file_exists(data_file) {
        println!("Data file is required and does not exist");
        return;
    }

    println!(
        "Uploading data file: {:?} to course_id: {}, chapter_id: {}, and topic_id: {}",
        data_file, course_id, chapter_id, topic_id
    );

    let file_contents = match read_file_contents(data_file) {
        FileOpsResult::Success(c) => c,
        FileOpsResult::Error(e) => {
            println!("Failed to read data file: {:?}", e);
            return;
        }
    };

    let questions: Vec<QuestionEntity> = match serde_json::from_str(file_contents.as_str()) {
        Ok(m) => m,
        Err(e) => {
            println!("Failed to parse json: {:?}", e);
            return;
        }
    };

    for question in questions {
        match question.create(pool).await {
            EntityResult::Success(_) => println!("Question created successfully"),
            EntityResult::Error(e) => println!("Failed to create question: {:?}", e),
        }
    }
}
