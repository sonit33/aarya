mod handlers;
use std::path::PathBuf;
use clap::{ Parser, Subcommand };
use handlers::{ handle_autogen, handle_upload, handle_validate };

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
            handle_validate(schema_file, data_file).await;
        }
        Some(Commands::Autogen { screenshot_path, output_path, prompt_path }) => {
            handle_autogen(screenshot_path, output_path, prompt_path).await;
        }
        Some(Commands::Upload { course_id, chapter_id, data_file }) => {
            handle_upload(course_id, chapter_id, data_file).await;
        }
        None => {
            println!("No command provided. Use aarya_cli --help to see available commands.");
        }
    }
}
