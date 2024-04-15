use aarya_utils::{
    api::models::question_model::QuestionModel,
    environ::Environ,
    file_ops::{read_file_contents, write_to_file, FileOpsResult},
    image_ops::{encode_to_base64, ImageOpsResult},
    json_ops::{self, validate_json_text, JsonOpsResult},
    openai::{
        completion_model::CompletionResponse,
        openai_ops::{prep_header, prep_payload, prep_payload_wo_image, send_request, OpenAiResponse, Payload},
    },
};
use clap::{Parser, Subcommand};
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
    Questions {
        /// path to the json schema
        #[arg(long, value_name = "FILE")]
        schema_file: Option<PathBuf>,
        /// path to the json data
        #[arg(long, value_name = "FILE")]
        data_file: Option<PathBuf>,
    },
    /// autogenerate questions using OpenAI API calls using a prompt template and a screenshot
    Autogenerate {
        /// course id
        #[arg(long, default_value_t = 2)]
        course_id: u8,

        /// chapter id
        #[arg(long, default_value_t = 2)]
        chapter_id: u8,

        /// path to the screenshot file
        #[arg(long, value_name = "FILE")]
        screenshot_path: Option<PathBuf>,
    },
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    match &cli.command {
        Some(Commands::Questions { schema_file, data_file }) => {
            match (schema_file, data_file) {
                (Some(schema), Some(data)) => {
                    println!("Uploading questions to database");
                    // validate the data file against the schema file
                    match json_ops::validate_json_file(schema.to_str().unwrap(), data.to_str().unwrap()) {
                        JsonOpsResult::Success(_) => {
                            println!("Validation successful");
                            match json_ops::json_to_vec::<QuestionModel>(&data.to_str().unwrap()) {
                                JsonOpsResult::Success(questions) => {
                                    println!("Questions: {:?}", questions);
                                    let client = reqwest::Client::new();
                                    for question in questions {
                                        println!("Uploading question: {:?}", question);
                                        match client.post("http://127.0.0.1:8080/question").json(&question).send().await {
                                            Ok(_) => {
                                                println!("Question uploaded successfully");
                                            }
                                            Err(e) => {
                                                println!("Failed to upload question: {:?}", e);
                                            }
                                        }
                                    }
                                }
                                JsonOpsResult::Error(e) => {
                                    println!("Failed to parse questions: {:?}", e);
                                }
                            }
                        }
                        JsonOpsResult::Error(e) => {
                            println!("Validation failed: {:?}", e);
                        }
                    }
                }
                _ => {
                    println!("Missing required arguments");
                }
            }
        }
        Some(Commands::Autogenerate {
            course_id,
            chapter_id,
            screenshot_path,
        }) => {
            let env = Environ::default();

            let header_map = match prep_header(env.openai_key) {
                OpenAiResponse::Success(h) => h,
                OpenAiResponse::Error(_) => {
                    panic!("Failed to prepare OpenAI header");
                }
            };

            let folder = format!(".temp-data/co-{course_id}-ch-{chapter_id}");
            let prompt_path = format!(".prompts/co-{course_id}-ch-{chapter_id}/prompt.txt");
            let schema_path = ".schema/question-schema.json";

            println!("reading {prompt_path}");
            // read the prompt file
            let prompt = match read_file_contents(&prompt_path) {
                FileOpsResult::Success(p) => p,
                FileOpsResult::Error(e) => {
                    println!("Failed to read prompt file: [{:?}]", e);
                    return;
                }
            };

            // encode the image to base64 if path is provided
            let mut encoded_image = String::new();
            if !screenshot_path.is_none() {
                let image_path = screenshot_path.as_ref().unwrap().to_str().unwrap();
                println!("reading {}", image_path);
                encoded_image = match encode_to_base64(image_path) {
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
                    let completion_res: CompletionResponse = match serde_json::from_str(&r) {
                        Ok(completion_res) => completion_res,
                        Err(e) => {
                            println!("Error parsing to json: {:?}", e);
                            return;
                        }
                    };
                    println!("recieved response");
                    match write_to_file(format!("{folder}/{}.txt", &completion_res.id).as_str(), &r) {
                        FileOpsResult::Success(_) => {
                            println!("written to file");
                            let content = completion_res.choices[0].message.content.clone();
                            // validate content against the schema
                            match validate_json_text(schema_path, content.as_str()) {
                                JsonOpsResult::Success(_) => {
                                    println!("json file is valid");
                                    let questions: Vec<QuestionModel> = match serde_json::from_str(&content) {
                                        Ok(questions) => questions,
                                        Err(e) => {
                                            println!("Failed to parse questions: {:?}", e);
                                            return;
                                        }
                                    };
                                    println!("saving question to database");
                                    // call API to save questions to database
                                    let client = reqwest::Client::new();
                                    for mut question in questions {
                                        //question_id required but its value doesn't matter
                                        question.question_id = 0;
                                        question.course_id = *course_id as u32;
                                        question.chapter_id = *chapter_id as u32;
                                        question.id_hash = "not-set".to_string();
                                        match client.post("http://127.0.0.1:8080/question").json(&question).send().await {
                                            Ok(_) => {
                                                println!("Question uploaded successfully");
                                            }
                                            Err(e) => {
                                                println!("Failed to upload question: {:?}", e);
                                            }
                                        }
                                    }
                                }
                                JsonOpsResult::Error(e) => {
                                    println!("Failed to validate json: {:?}", e);
                                    return;
                                }
                            }
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
        _ => {
            println!("Missing required arguments");
        }
    }
}
