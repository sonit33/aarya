use std::path::PathBuf;

use aarya_models::database::question::{ Question, QuestionFromJson };
use aarya_utils::{
    db_ops::setup_durable_database,
    environ::Environ,
    hasher,
    json_ops,
    random::generate_guid,
};
use clap::{ Parser, Subcommand };
use serde_json::json;

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
                (Some(schema_file), Some(data_file)) => {
                    match
                        json_ops::validate_json_file(
                            schema_file.to_str().unwrap(),
                            data_file.to_str().unwrap()
                        )
                    {
                        Ok(r) =>
                            match r {
                                true => {
                                    let env_default = Environ::default();
                                    match
                                        setup_durable_database(
                                            env_default.db_connection_string
                                        ).await
                                    {
                                        Ok(pool) => {
                                            match
                                                json_ops::json_to_vec::<QuestionFromJson>(
                                                    &data_file.to_str().unwrap()
                                                )
                                            {
                                                Ok(questions) => {
                                                    for question in questions {
                                                        let mut q = Question::new();
                                                        q.course_id = 2;
                                                        q.chapter_id = 2;
                                                        q.id_hash = hasher::fast_hash(
                                                            generate_guid(8).as_str()
                                                        );
                                                        q.q_text = question.q_text.to_string();
                                                        q.choices = json!(question.choices);
                                                        q.answers = json!(question.answers);
                                                        q.a_explanation = question.a_explanation;
                                                        q.a_hint = question.a_hint;
                                                        q.difficulty = question.difficulty;
                                                        q.diff_reason = question.diff_reason;
                                                        match q.create_if(&pool).await {
                                                            Ok(q) => {
                                                                match q {
                                                                    Some(q) => {
                                                                        println!(
                                                                            "Question created: {:?}",
                                                                            q
                                                                        );
                                                                    }
                                                                    None => {
                                                                        println!(
                                                                            "Question already exists"
                                                                        );
                                                                    }
                                                                }
                                                            }
                                                            Err(e) => {
                                                                println!("Failed to create question: [{}]", e);
                                                            }
                                                        }
                                                    }
                                                }
                                                Err(e) => {
                                                    println!("Failed to convert json to vector of questions: [{}]", e)
                                                }
                                            }
                                        }
                                        Err(e) => {
                                            println!("Failed to establish database connection: [{}]", e)
                                        }
                                    }
                                }
                                false => {
                                    println!("the data file is invalid");
                                }
                            }
                        Err(e) => {
                            println!("Failed to validate the data file: [{}]", e);
                        }
                    }
                }
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
