use aarya_utils::{
    environ::Environ,
    file_ops::{ file_exists, read_file_contents, write_to_file, FileOpsResult },
    image_ops::{ encode_to_base64, ImageOpsResult },
    json_ops::{ self, JsonOpsResult },
    models::question_model::QuestionModel,
    openai::{
        completion_model::CompletionResponse,
        openai_ops::{
            prep_header,
            prep_payload,
            prep_payload_wo_image,
            send_request,
            OpenAiResponse,
            Payload,
        },
    },
};
use clap::{ Parser, Subcommand };

use std::path::PathBuf;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// upload questions from json files to database
    /// aarya_cli validate --schema-file --data-file
    Validate {
        /// path to the json schema
        #[arg(long, value_name = "FILE")]
        schema_file: PathBuf,

        /// path to the json data
        #[arg(long, value_name = "FILE")]
        data_file: PathBuf,
    },
    /// autogenerate questions using OpenAI API calls using a prompt template and a screenshot
    /// aarya_cli autogen --screenshot-path --output-path --prompt-path
    Autogen {
        /// path to the screenshot file
        #[arg(long, value_name = "FILE")]
        screenshot_path: Option<PathBuf>,

        #[arg(long, value_name = "FILE")]
        output_path: Option<PathBuf>,

        #[arg(long, value_name = "FILE")]
        prompt_path: PathBuf,
    },
    /// upload questions from json files to database
    /// aarya_cli upload --data-file --chapter-id --course-id
    Upload {
        /// course id
        #[arg(long)]
        course_id: u8,

        /// chapter id
        #[arg(long)]
        chapter_id: u8,

        /// path to the json data
        #[arg(long, value_name = "FILE")]
        data_file: PathBuf,
    },
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    match &cli.command {
        Some(Commands::Validate { schema_file, data_file }) => {
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
        Some(Commands::Autogen { screenshot_path, output_path, prompt_path }) => {
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

            let payload: Payload;
            if encoded_image.is_empty() {
                payload = prep_payload_wo_image(prompt);
            } else {
                payload = prep_payload(encoded_image, prompt);
            }

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
                            return;
                        }
                    };
                }
                OpenAiResponse::Error(e) => {
                    println!("Failed to send request to OpenAI API: {:?}", e);
                    return;
                }
            }
        }
        Some(Commands::Upload { course_id, chapter_id, data_file }) => {
            let data_file = data_file.to_str().unwrap();
            if !file_exists(data_file) {
                println!("Data file is required and does not exist");
                return;
            }

            println!(
                "Uploading data file: {:?} to course_id: {} and chapter_id: {}",
                data_file,
                course_id,
                chapter_id
            );

            let file_contents = match read_file_contents(data_file) {
                FileOpsResult::Success(c) => c,
                FileOpsResult::Error(e) => {
                    println!("Failed to read data file: {:?}", e);
                    return;
                }
            };

            let questions: Vec<QuestionModel> = match serde_json::from_str(file_contents.as_str()) {
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
                question.id_hash = "not-set".to_string();
                match client.post("http://localhost:8080/question").json(&question).send().await {
                    Ok(r) => {
                        println!("Uploaded question: {:?}", r.status());
                    }
                    Err(e) => {
                        println!("Failed to upload question: {:?}", e);
                    }
                }
            }
        }
        None => {
            println!("No command provided. Use aarya_cli --help to see available commands.");
        }
    }
}
