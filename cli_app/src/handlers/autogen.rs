use aarya_utils::{
    environ::Environ,
    file_ops::{file_exists, read_file_contents, write_to_file, FileOpsResult},
    image_ops::{encode_to_base64, ImageOpsResult},
    openai::{
        completion_model::CompletionResponse,
        openai_ops::{prep_header, prep_payload, prep_payload_wo_image, send_request, OpenAiResponse},
    },
};

use std::path::{Path, PathBuf};

pub async fn autogen(screenshot_path: &Option<PathBuf>, output_path: &Option<PathBuf>, prompt_path: &Path) {
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
