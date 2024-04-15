use aarya_utils::{ api::models::question_model::QuestionModel, json_ops::{ self, JsonOpsResult } };
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
                    match
                        json_ops::validate_json_file(
                            schema.to_str().unwrap(),
                            data.to_str().unwrap()
                        )
                    {
                        JsonOpsResult::Success(_) => {
                            println!("Validation successful");
                            match json_ops::json_to_vec::<QuestionModel>(&data.to_str().unwrap()) {
                                JsonOpsResult::Success(questions) => {
                                    println!("Questions: {:?}", questions);
                                    let client = reqwest::Client::new();
                                    for question in questions {
                                        println!("Uploading question: {:?}", question);
                                        match
                                            client
                                                .post("http://127.0.0.1:8080/question")
                                                .json(&question)
                                                .send().await
                                        {
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
                _ => (),
            }
        }
        Some(Commands::Autogenerate { course_id, chapter_id, screenshot_path }) => {}
        _ => (),
    }
}
