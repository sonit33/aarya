use std::path::PathBuf;

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

fn main() {
    let cli = Cli::parse();
    match &cli.command {
        Some(Commands::Questions {
            schema_file,
            data_file,
        }) => {
            match (schema_file, data_file) {
                // Both schema_file and data_file are Some
                (Some(_), Some(_)) => {
                    println!("processing...");
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
