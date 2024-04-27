use std::path::PathBuf;

use aarya_utils::json_ops::{json_to_vec, JsonOpsResult};
use models::{chapters::ChapterEntity, courses::CourseEntity, result_types::EntityResult};
use sqlx::MySqlPool;

pub async fn run_seeder(
    courses_file: &Option<PathBuf>,
    chapters_file: &Option<PathBuf>,
    topics_file: &Option<PathBuf>,
    questions_file: &Option<PathBuf>,
    pool: &MySqlPool,
) {
    if courses_file.is_some() {
        println!("Ensure validating the file contents. Now processing the courses file: {:?}", courses_file);
        let courses = match json_to_vec::<CourseEntity>(courses_file.as_ref().unwrap().to_str().unwrap()) {
            JsonOpsResult::Success(courses) => courses,
            JsonOpsResult::Error(e) => {
                println!("Failed to read courses file: {:?}", e);
                return;
            }
        };
        for course in courses {
            match course.create(pool).await {
                EntityResult::Success(r) => println!("Course created successfully: {:?}", r),
                EntityResult::Error(e) => println!("Failed to create course: {:?}", e),
            }
        }
    } else if chapters_file.is_some() {
        println!("Processing chapters file");
        println!("Ensure validating the file contents. Now processing the courses file: {:?}", chapters_file);
        let chapters = match json_to_vec::<ChapterEntity>(chapters_file.as_ref().unwrap().to_str().unwrap()) {
            JsonOpsResult::Success(chapters) => chapters,
            JsonOpsResult::Error(e) => {
                println!("Failed to read chapters file: {:?}", e);
                return;
            }
        };
        for chapter in chapters {
            match chapter.create_chapter(pool).await {
                EntityResult::Success(r) => println!("Chapter created successfully: {:?}", r),
                EntityResult::Error(e) => println!("Failed to create chapter: {:?}", e),
            }
        }
    } else if topics_file.is_some() {
        println!("Processing topics file");
        // verify the file exists
        // validate the file against the schema
        // read topics from the file
        // save topics to the database
    } else if questions_file.is_some() {
        println!("Processing questions file");
        // verify the file exists
        // validate the file against the schema
        // read questions from the file
        // save questions to the database
    } else {
        println!("No file provided to seed the database");
    }
}
