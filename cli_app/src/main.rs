pub mod handlers;

use aarya_utils::environ::Environ;
use clap::{Parser, Subcommand};
use dotenv::from_filename;
use handlers::{autogen::autogen, seeder::seeder, uploader::upload, validate::validate};
use sqlx::MySqlPool;
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
    /// aarya_cli upload --data-file --chapter-id --course-id --topic-id
    Upload {
        /// course id
        #[arg(long)]
        course_id: u32,

        /// chapter id
        #[arg(long)]
        chapter_id: u32,

        /// topic id
        #[arg(long)]
        topic_id: u32,

        /// path to the json data
        #[arg(long, value_name = "FILE")]
        data_file: PathBuf,
    },
    Seeder {
        #[arg(long, value_name = "FILE")]
        courses_file: Option<PathBuf>,
        #[arg(long, value_name = "FILE")]
        chapters_file: Option<PathBuf>,
        #[arg(long, value_name = "FILE")]
        topics_file: Option<PathBuf>,
    },
}

#[tokio::main]
async fn main() {
    let env_file = if cfg!(debug_assertions) { ".env.dev" } else { ".env.prod" };
    from_filename(env_file).ok();
    let env_default = Environ::default();
    let database_url = format!("{}/{}", env_default.db_connection_string, env_default.db_name);
    let pool = MySqlPool::connect(database_url.as_str()).await.expect("Failed to connect to database");

    let cli = Cli::parse();
    match &cli.command {
        Some(Commands::Validate { schema_file, data_file }) => {
            validate(schema_file, data_file).await;
        }
        Some(Commands::Autogen {
            screenshot_path,
            output_path,
            prompt_path,
        }) => {
            autogen(screenshot_path, output_path, prompt_path).await;
        }
        Some(Commands::Upload {
            course_id,
            chapter_id,
            topic_id,
            data_file,
        }) => {
            upload(*course_id, *chapter_id, *topic_id, data_file, &pool).await;
        }
        Some(Commands::Seeder {
            courses_file,
            chapters_file,
            topics_file,
        }) => {
            seeder(courses_file, chapters_file, topics_file, &pool).await;
        }
        None => {
            println!("No command provided. Use aarya_cli --help to see available commands.");
        }
    }
}
