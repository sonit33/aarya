use std::path::Path;

use aarya_utils::{
    file_ops::{make_dir, FileOpsResult},
    random::generate_timestamp,
};
use models::{
    chapters::ChapterEntity,
    courses::{CourseDetailQueryModel, CourseEntity},
    result_types::EntityResult,
};
use serde::{Deserialize, Serialize};
use sqlx::MySqlPool;

use super::autogener::{run_autogen, AutogenArgs};

#[derive(Debug, Serialize, Deserialize)]
pub struct ManifestModel {
    pub file_path: String,
    pub model: CourseDetailQueryModel,
}

pub async fn run_batch(
    course_id: Option<u32>,
    chapter_id: Option<u32>,
    count: u32,
    prompt_path: &Path,
    pool: &MySqlPool,
) {
    let session_id = generate_timestamp();

    if course_id.is_some() && chapter_id.is_none() {
        // autogen given course
        let mut course = CourseEntity::new();
        course.course_id = course_id.unwrap();
        let courses = match course.find_all(pool).await {
            EntityResult::Success(c) => c,
            EntityResult::Error(e) => {
                eprintln!("Error: {:?}", e);
                return;
            }
        };
        let data_folder = format!("./.temp-data/course-{}-{:?}", course.course_id, session_id);
        save_to_file(courses, count, prompt_path, data_folder).await;
    }

    if chapter_id.is_some() && course_id.is_some() {
        // autogen given chapter
        let mut chapter = ChapterEntity::new();
        chapter.course_id = course_id.unwrap();
        chapter.chapter_id = chapter_id.unwrap();
        let chapters = match chapter.find_all(pool).await {
            EntityResult::Success(c) => c,
            EntityResult::Error(e) => {
                eprintln!("Error: {:?}", e);
                return;
            }
        };
        let data_folder = format!("./.temp-data/course-{}-chapter-{}-{:?}", chapter.course_id, chapter.chapter_id, session_id);
        save_to_file(chapters, count, prompt_path, data_folder).await;
    }

    if chapter_id.is_none() && course_id.is_none() {
        println!("No course or chapter provided");
    }
}

async fn save_to_file(
    courses: Vec<CourseDetailQueryModel>,
    count: u32,
    prompt_path: &Path,
    data_folder: String,
) {
    match make_dir(data_folder.as_str()) {
        FileOpsResult::Success(_) => {
            println!("Using {} to save the generated questions", data_folder);
        }
        FileOpsResult::Error(_) => {
            eprintln!("Failed to create data folder");
            return;
        }
    }

    let mut counter = 0;
    let len = courses.len() as u32;
    let mut manifest: Vec<ManifestModel> = vec![];
    for c in courses {
        let args = AutogenArgs {
            course_name: c.course_name.clone(),
            course_id: c.course_id,
            chapter_id: c.chapter_id,
            chapter_name: c.chapter_name.clone(),
            topic_id: c.topic_id,
            topic_name: c.topic_name.clone(),
            count,
        };
        let output_file = run_autogen(&None, prompt_path, &args, data_folder.as_str()).await;
        manifest.push(ManifestModel {
            file_path: output_file.unwrap(),
            model: c,
        });
        counter += 1;
        println!("Finished {} of {}", counter, len);
        println!("----------------------------------");
    }

    // serialize the manifest
    let manifest_file = format!("{}/manifest.json", data_folder);
    let manifest_contents = serde_json::to_string(&manifest).unwrap();
    match aarya_utils::file_ops::write_to_file(manifest_file.as_str(), manifest_contents.as_str()) {
        FileOpsResult::Success(_) => println!("Manifest file created at: {manifest_file}"),
        FileOpsResult::Error(_) => eprintln!("Failed to create manifest file"),
    }
}

/*
./aarya_cli autogen \
--course-id 1002 \
--course-name "AP Computer Science A" \
--chapter-name "Primitive Types" \
--chapter-id 1010 \
--topic-name "Mathematical Operations" \
--topic-id 1004 \
--count 10 \
--prompt-path ../.prompts/prompt.txt
*/
