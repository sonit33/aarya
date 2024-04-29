use std::path::{Path, PathBuf};

use aarya_utils::{
    environ::Environ,
    file_ops::{file_exists, read_file_contents, write_to_file, FileOpsResult},
    image_ops::{encode_to_base64, ImageOpsResult},
    openai::{
        completion_model::CompletionResponse,
        openai_ops::{prep_header, prep_payload, prep_payload_wo_image, send_request, OpenAiResponse},
    },
    random::generate_timestamp,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct AutogenArgs {
    pub course_name: String,
    pub chapter_name: String,
    pub topic_name: String,
    pub course_id: u32,
    pub chapter_id: u32,
    pub topic_id: u32,
    pub count: u32,
}

/// this function uses courses, chapters, and topics from the database
/// to create copies of the prompt template
/// that is used to generate questions using the OpenAI API
/// the generated questions are saved in separate files
/// that can be used to upload to the database
pub async fn run_autogen(
    screenshot_path: &Option<PathBuf>,
    prompt_path: &Path,
    args: &AutogenArgs,
    output_folder: &str,
) -> Option<String> {
    let session_id = generate_timestamp().to_string();
    if screenshot_path.is_none() {
        println!("Screenshot path not provided");
    }

    println!("Using {output_folder} to save the generated questions");

    let prompt_path = prompt_path.to_str().unwrap();

    if !file_exists(prompt_path) {
        println!("Prompt file is required and it does not exist");
        return None;
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
    let mut prompt = match read_file_contents(prompt_path) {
        FileOpsResult::Success(p) => p,
        FileOpsResult::Error(e) => {
            println!("Failed to read prompt file: [{:?}]", e);
            return None;
        }
    };

    // replace the placeholders in the prompt with the actual values
    prompt = prompt
        .replace("{{num_questions}}", &args.count.to_string())
        .replace("{{course_name}}", &args.course_name)
        .replace("{{chapter_name}}", &args.chapter_name)
        .replace("{{topic_name}}", &args.topic_name)
        .replace("{{course_id}}", &args.course_id.to_string())
        .replace("{{chapter_id}}", &args.chapter_id.to_string())
        .replace("{{topic_id}}", &args.topic_id.to_string());

    // encode the image to base64 if path is provided
    let mut encoded_image = String::new();
    let screenshot_path = screenshot_path.as_ref().unwrap();
    println!("Screenshot path: {:?}", screenshot_path);
    if screenshot_path.is_file() {
        let screenshot_path = screenshot_path.to_str().unwrap();
        println!("Encoding image {}", screenshot_path);
        encoded_image = match encode_to_base64(screenshot_path) {
            ImageOpsResult::Success(img) => img,
            ImageOpsResult::Error(e) => {
                println!("Failed to encode image to base64: {:?}", e);
                return None;
            }
        };
    }

    let payload = if encoded_image.is_empty() {
        prompt = prompt.replace("{{screenshot}}", "");
        prep_payload_wo_image(prompt)
    } else {
        prompt = prompt.replace("{{screenshot}}", "Use the attached screenshot as an example to generate variants of the question.");
        prep_payload(encoded_image, prompt)
    };

    println!("sending request to OpenAI API");
    let output_file = format!("{output_folder}/{session_id}.json");
    match send_request(header_map, payload).await {
        OpenAiResponse::Success(r) => {
            let message: CompletionResponse = match serde_json::from_str(&r) {
                Ok(res) => res,
                Err(e) => {
                    println!("Error parsing to json: {:?}", e);
                    return None;
                }
            };

            let contents = &message.choices[0].message.content;

            match write_to_file(output_file.as_str(), contents) {
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

    Some(output_file)
}
