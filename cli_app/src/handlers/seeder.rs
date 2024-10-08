use std::path::PathBuf;

use aarya_utils::json_ops::{json_to_vec, JsonOpsResult};
use models::{
    blogs::{AuthorEntity, TagEntity},
    chapters::ChapterEntity,
    courses::CourseEntity,
    result_types::EntityResult,
    topics::TopicEntity,
};
use sqlx::MySqlPool;

pub async fn run_seeder(
    courses_file: &Option<PathBuf>,
    chapters_file: &Option<PathBuf>,
    topics_file: &Option<PathBuf>,
    authors_file: &Option<PathBuf>,
    tags_file: &Option<PathBuf>,
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
        println!("Ensure validating the file contents. Now processing the topics file: {:?}", topics_file);
        let topics = match json_to_vec::<TopicEntity>(topics_file.as_ref().unwrap().to_str().unwrap()) {
            JsonOpsResult::Success(topics) => topics,
            JsonOpsResult::Error(e) => {
                println!("Failed to read topics file: {:?}", e);
                return;
            }
        };
        for topic in topics {
            match topic.create_topic(pool).await {
                EntityResult::Success(r) => println!("Topic created successfully: {:?}", r),
                EntityResult::Error(e) => println!("Failed to create topic: {:?}", e),
            }
        }
    } else if authors_file.is_some() {
        println!("Processing authors file");
        println!("Ensure validating the file contents. Now processing the authors file: {:?}", authors_file);
        let authors = match json_to_vec::<AuthorEntity>(authors_file.as_ref().unwrap().to_str().unwrap()) {
            JsonOpsResult::Success(authors) => authors,
            JsonOpsResult::Error(e) => {
                println!("Failed to read authors file: {:?}", e);
                return;
            }
        };
        for author in authors {
            match author.create(pool).await {
                EntityResult::Success(r) => println!("Author created successfully: {:?}", r),
                EntityResult::Error(e) => println!("Failed to create author: {:?}", e),
            }
        }
    } else if tags_file.is_some() {
        println!("Processing tags file");
        println!("Ensure validating the file contents. Now processing the tags file: {:?}", tags_file);
        let tags = match json_to_vec::<TagEntity>(tags_file.as_ref().unwrap().to_str().unwrap()) {
            JsonOpsResult::Success(tags) => tags,
            JsonOpsResult::Error(e) => {
                println!("Failed to read tags file: {:?}", e);
                return;
            }
        };
        for tag in tags {
            match tag.create(pool).await {
                EntityResult::Success(r) => println!("Tag created successfully: {:?}", r),
                EntityResult::Error(e) => println!("Failed to create tag: {:?}", e),
            }
        }
    } else {
        println!("No file provided to seed the database");
    }
}
