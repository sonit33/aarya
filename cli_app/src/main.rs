pub mod handlers;

use aarya_utils::environ::Environ;
use clap::{Parser, Subcommand};
use dotenv::from_filename;
use handlers::{
    autogener::{run_autogen, AutogenArgs},
    batchgener::run_batch,
    batchuploader::run_batch_uploads,
    seeder::run_seeder,
    uploader::run_upload,
    validator::run_validate,
};
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
    Validate {
        /// path to the json schema
        #[arg(long, value_name = "FILE")]
        schema_file: PathBuf,

        /// path to the json data
        #[arg(long, value_name = "FILE")]
        data_file: PathBuf,
    },
    /// autogenerate questions using OpenAI API calls using a prompt template and a screenshot
    Autogen {
        /// path to the screenshot file
        #[arg(long, value_name = "FILE")]
        screenshot_path: Option<PathBuf>,

        /// number of questions to generate
        #[arg(long)]
        count: u32,

        /// full name of a valid course
        #[arg(long)]
        course_name: String,

        /// valid course id
        #[arg(long)]
        course_id: u32,

        /// full name of a valid chapter
        #[arg(long)]
        chapter_name: String,

        /// valid chapter id
        #[arg(long)]
        chapter_id: u32,

        /// full name of a valid topic
        #[arg(long)]
        topic_name: String,

        /// valid topic id
        #[arg(long)]
        topic_id: u32,

        /// path to the prompt file
        #[arg(long, value_name = "FILE")]
        prompt_path: PathBuf,
    },
    /// upload questions from json files to database
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
    /// seed the database with courses, chapters, topics, and questions
    /// run `aarya_cli validate --schema-file schema.json --data-file data.json`
    /// before running this command
    Seeder {
        #[arg(long, value_name = "FILE")]
        courses_file: Option<PathBuf>,
        #[arg(long, value_name = "FILE")]
        chapters_file: Option<PathBuf>,
        #[arg(long, value_name = "FILE")]
        topics_file: Option<PathBuf>,
        #[arg(long, value_name = "FILE")]
        authors_file: Option<PathBuf>,
        #[arg(long, value_name = "FILE")]
        tags_file: Option<PathBuf>,
    },
    /// calls `autogen` in a loop using courses, chapters, and topics from the database
    Batchgen {
        /// call `autogen` for all chapters and topics in a course
        #[arg(long)]
        course_id: Option<u32>,

        /// call `autogen` for all topics in a chapter
        #[arg(long)]
        chapter_id: Option<u32>,

        /// number of questions to generate for each topic
        #[arg(long)]
        count: u32,

        /// path to the prompt file
        #[arg(long, value_name = "FILE")]
        prompt_path: PathBuf,

        /// path to the screenshot folder
        #[arg(long, value_name = "FILE")]
        screenshot_path: Option<PathBuf>,
    },
    /// processes and loads all json files in a directory to database
    BatchUpload {
        /// question json schema file path
        #[arg(long, value_name = "FILE")]
        schema_file: PathBuf,

        /// directory path to the json data
        #[arg(long, value_name = "FILE")]
        directory: PathBuf,
    },
    /// process and save a blog post file
    Blog {
        #[arg(long, value_name = "FILE")]
        post_file: PathBuf,
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
            run_validate(schema_file, data_file).await;
        }
        Some(Commands::Autogen {
            screenshot_path,
            course_name,
            chapter_name,
            topic_name,
            count,
            prompt_path,
            course_id,
            chapter_id,
            topic_id,
        }) => {
            let args = AutogenArgs {
                course_name: course_name.to_string(),
                chapter_name: chapter_name.to_string(),
                topic_name: topic_name.to_string(),
                course_id: *course_id,
                chapter_id: *chapter_id,
                topic_id: *topic_id,
                count: *count,
            };
            run_autogen(screenshot_path, prompt_path, &args, "./temp-data").await;
        }
        Some(Commands::Upload {
            course_id,
            chapter_id,
            topic_id,
            data_file,
        }) => {
            run_upload(*course_id, *chapter_id, *topic_id, data_file, &pool).await;
        }
        Some(Commands::Seeder {
            courses_file,
            chapters_file,
            topics_file,
            authors_file,
            tags_file,
        }) => {
            run_seeder(courses_file, chapters_file, topics_file, authors_file, tags_file, &pool).await;
        }
        Some(Commands::Batchgen {
            course_id,
            chapter_id,
            count,
            prompt_path,
            screenshot_path,
        }) => {
            run_batch(*course_id, *chapter_id, *count, prompt_path, screenshot_path, &pool).await;
        }
        Some(Commands::BatchUpload { schema_file, directory }) => {
            run_batch_uploads(schema_file, directory, &pool).await;
        }
        Some(Commands::Blog { post_file }) => {
            println!("Processing blog post file: {:?}", post_file);
        }
        None => {
            println!("No command provided. Use aarya_cli --help to see available commands.");
        }
    }
}
