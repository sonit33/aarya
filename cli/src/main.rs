pub mod question_ops;

use std::path::PathBuf;

use aarya_models::database::question::QuestionFromJson;
use aarya_utils::{
    db_ops::DbOps,
    environ::Environ,
    file_ops::{read_file_contents, write_to_file},
    image_ops::encode_to_base64,
    json_ops::{self, validate_json_text},
    openai_ops::{
        completion_model::CompletionResponse,
        openai_ops::{prep_header, prep_payload, prep_payload_wo_image, send_request, Payload},
    },
};
use clap::{Parser, Subcommand};
use question_ops::save;

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
                // Both schema_file and data_file are Some
                (Some(schema_file), Some(data_file)) => match json_ops::validate_json_file(schema_file.to_str().unwrap(), data_file.to_str().unwrap()) {
                    Ok(r) => match r {
                        true => match json_ops::json_to_vec::<QuestionFromJson>(&data_file.to_str().unwrap()) {
                            Ok(questions) => save(questions, DbOps).await,
                            Err(e) => println!("Failed to convert json to vector of questions: [{}]", e),
                        },
                        false => {
                            println!("the data file is invalid");
                        }
                    },
                    Err(e) => {
                        println!("Failed to validate the data file: [{}]", e);
                    }
                },
                // Only schema_file is Some
                (Some(_), None) => {
                    println!("data file missing");
                }
                // Only data_file is Some
                (None, Some(_)) => {
                    println!("schema file missing");
                }
                // Both schema_file and data_file are None
                (None, None) => {
                    println!("schema and data files missing");
                }
            }
        }
        Some(Commands::Autogenerate {
            course_id,
            chapter_id,
            screenshot_path,
        }) => {
            let env = Environ::default();
            let h = prep_header(env.openai_key).unwrap();

            let folder = format!("../.temp-data/co-{course_id}-ch-{chapter_id}");
            let prompt_path = format!("{folder}/prompt.txt");
            let schema_path = "../.schema/question-schema.json";

            println!("reading {prompt_path}");
            // read the prompt file
            let prompt = match read_file_contents(&prompt_path) {
                Ok(prompt) => prompt,
                Err(e) => {
                    panic!("Error: {}", e);
                }
            };

            // encode the image to base64 if path is provided
            let mut encoded_image = String::new();
            if !screenshot_path.is_none() {
                println!("reading {}", screenshot_path.as_ref().unwrap().to_str().unwrap());
                encoded_image = match encode_to_base64(screenshot_path.as_ref().unwrap().to_str().unwrap()) {
                    Ok(encoded_image) => encoded_image,
                    Err(e) => {
                        panic!("Error: {}", e);
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
            match send_request(h, payload).await {
                Ok(r) => {
                    let completion_res: CompletionResponse = match serde_json::from_str(&r) {
                        Ok(completion_res) => completion_res,
                        Err(e) => {
                            panic!("Error: {}", e);
                        }
                    };
                    println!("recieved response");
                    match write_to_file(format!("{folder}/{}.txt", &completion_res.id).as_str(), &r) {
                        Ok(_) => {
                            println!("written to file");
                            let content = completion_res.choices[0].message.content.clone();
                            // validate content against the schema
                            match validate_json_text(schema_path, content.as_str()) {
                                Ok(_) => {
                                    println!("json file is valid");
                                    let questions: Vec<QuestionFromJson> = match serde_json::from_str(&content) {
                                        Ok(questions) => questions,
                                        Err(e) => {
                                            panic!("Error: {}", e);
                                        }
                                    };
                                    println!("saving question to database");
                                    save(questions, DbOps).await;
                                }
                                Err(e) => {
                                    println!("Failed to validate the data file: [{}]", e);
                                }
                            }
                        }
                        Err(e) => {
                            eprintln!("Error: {:?}", e);
                        }
                    };
                }
                Err(e) => {
                    eprintln!("Error: {:?}", e);
                }
            }
        }
        _ => (),
    }
}
