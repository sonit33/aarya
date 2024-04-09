pub mod question_ops;

use crate::question_ops::save;
use std::path::PathBuf;

use aarya_models::database::question::QuestionFromJson;
use aarya_utils::json_ops;
use clap::{Parser, Subcommand};

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
        #[arg(short, long, value_name = "FILE")]
        schema_file: Option<PathBuf>,
        /// path to the json data
        #[arg(short, long, value_name = "FILE")]
        data_file: Option<PathBuf>,
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
        None => {}
    }
}
