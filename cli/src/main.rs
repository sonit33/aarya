pub mod question_ops;

use std::path::PathBuf;

use aarya_models::database::question::QuestionFromJson;
use aarya_utils::json_ops;
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
        /// path to the prompt file
        #[arg(long)]
        prompt_path: Option<String>,

        /// course id
        #[arg(long, default_value_t = 2)]
        course_id: u8,

        /// chapter id
        #[arg(long, default_value_t = 2)]
        chapter_id: u8,

        /// number of questions to generate
        #[arg(long, default_value_t = 10)]
        count: u8,

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
                            Ok(questions) => {
                                save(questions).await;
                            }
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
            prompt_path,
            course_id,
            chapter_id,
            count,
            screenshot_path,
        }) => {
            println!("Prompt path: {:?}", prompt_path);
            println!("Course ID: {}", course_id); // Assuming default values if not provided
            println!("Chapter ID: {}", chapter_id);
            println!("Count: {}", count);
            println!("Screenshot path: {:?}", screenshot_path);
        }
        _ => (),
    }
}
