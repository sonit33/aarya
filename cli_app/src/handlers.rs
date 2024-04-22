use aarya_utils::{
    environ::Environ,
    file_ops::{file_exists, read_file_contents, write_to_file, FileOpsResult},
    image_ops::{encode_to_base64, ImageOpsResult},
    json_ops::{self, JsonOpsResult},
    openai::{
        completion_model::CompletionResponse,
        openai_ops::{prep_header, prep_payload, prep_payload_wo_image, send_request, OpenAiResponse},
    },
};
use models::questions::QuestionQueryModel;

use std::path::{Path, PathBuf};

pub async fn handle_validate(schema_file: &Path, data_file: &Path) {
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

pub async fn handle_autogen(screenshot_path: &Option<PathBuf>, output_path: &Option<PathBuf>, prompt_path: &Path) {
    if screenshot_path.is_none() {
        println!("Screenshot path not provided");
    }

    let mut output_folder = "./.temp-data";

    if output_path.is_none() {
        println!("Output path not provided. Using the ./.temp-data directory");
    } else {
        output_folder = output_path.as_ref().unwrap().to_str().unwrap();
    }

    let prompt_path = prompt_path.to_str().unwrap();

    if !file_exists(prompt_path) {
        println!("Prompt file is required and it does not exist");
        return;
    }

    println!("Autogenerating questions using prompt file: {:?}", prompt_path);

    let env = Environ::default();

    let header_map = match prep_header(env.openai_key) {
        OpenAiResponse::Success(h) => h,
        OpenAiResponse::Error(_) => {
            panic!("Failed to prepare OpenAI header");
        }
    };

    println!("reading {prompt_path}");
    let prompt = match read_file_contents(&prompt_path) {
        FileOpsResult::Success(p) => p,
        FileOpsResult::Error(e) => {
            println!("Failed to read prompt file: [{:?}]", e);
            return;
        }
    };

    // encode the image to base64 if path is provided
    let mut encoded_image = String::new();
    if screenshot_path.is_some() {
        let screenshot_path = screenshot_path.as_ref().unwrap().to_str().unwrap();
        println!("reading {}", screenshot_path);
        encoded_image = match encode_to_base64(screenshot_path) {
            ImageOpsResult::Success(img) => img,
            ImageOpsResult::Error(_) => {
                panic!("Failed to encode image to base64");
            }
        };
    }

    let payload = if encoded_image.is_empty() {
        prep_payload_wo_image(prompt)
    } else {
        prep_payload(encoded_image, prompt)
    };

    println!("sending request to OpenAI API");
    match send_request(header_map, payload).await {
        OpenAiResponse::Success(r) => {
            let message: CompletionResponse = match serde_json::from_str(&r) {
                Ok(res) => res,
                Err(e) => {
                    println!("Error parsing to json: {:?}", e);
                    return;
                }
            };

            println!("recieved response");

            let output_file = format!("{output_folder}/{}.json", &message.id);

            match write_to_file(output_file.as_str(), &message.choices[0].message.content) {
                FileOpsResult::Success(_) => {
                    println!("{output_file}");
                }
                FileOpsResult::Error(e) => {
                    println!("Failed to write to file: {:?}", e);
                }
            };
        }
        OpenAiResponse::Error(e) => {
            println!("Failed to send request to OpenAI API: {:?}", e);
        }
    }
}

pub async fn handle_upload(course_id: &u8, chapter_id: &u8, data_file: &Path) {
    let data_file = data_file.to_str().unwrap();
    if !file_exists(data_file) {
        println!("Data file is required and does not exist");
        return;
    }

    println!("Uploading data file: {:?} to course_id: {} and chapter_id: {}", data_file, course_id, chapter_id);

    let file_contents = match read_file_contents(data_file) {
        FileOpsResult::Success(c) => c,
        FileOpsResult::Error(e) => {
            println!("Failed to read data file: {:?}", e);
            return;
        }
    };

    let questions: Vec<QuestionQueryModel> = match serde_json::from_str(file_contents.as_str()) {
        Ok(m) => m,
        Err(e) => {
            println!("Failed to parse json: {:?}", e);
            return;
        }
    };

    // call API to save questions to database
    let client = reqwest::Client::new();
    for mut question in questions {
        //question_id and id_hash required but their values do not matter
        question.question_id = 1;
        question.course_id = *course_id as u32;
        question.chapter_id = *chapter_id as u32;
        // replace the following with entities call
        match client.post("http://localhost:8080/question").json(&question).send().await {
            Ok(r) => {
                if r.status().is_success() {
                    println!("Saved question: {:?}", r.status());
                } else {
                    println!("Failed to save question: {:?}", r.status());
                }
            }
            Err(e) => {
                println!("Post request failed: {:?}", e);
            }
        }
    }
}
