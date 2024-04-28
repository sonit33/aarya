use std::path::Path;

use aarya_utils::{
    file_ops::{make_dir, FileOpsResult},
    random::generate_timestamp,
};
use models::{chapters::ChapterEntity, courses::CourseEntity, result_types::EntityResult};
use sqlx::MySqlPool;

use super::autogener::{run_autogen, AutogenArgs};

pub async fn run_batch(
    course_id: Option<u32>,
    chapter_id: Option<u32>,
    count: u32,
    prompt_path: &Path,
    pool: &MySqlPool,
) {
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

        let session_id = generate_timestamp();
        let data_folder = format!("./.temp-data/course-{}-{:?}", course.course_id, session_id);
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
            run_autogen(&None, prompt_path, &args, data_folder.as_str()).await;
            counter += 1;
            println!("Finished {} of {}", counter, len);
            println!("----------------------------------");
        }
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

        let session_id = generate_timestamp();
        let data_folder = format!("./.temp-data/course-{}-chapter-{}-{:?}", chapter.course_id, chapter.chapter_id, session_id);
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
        let len = chapters.len() as u32;
        for c in chapters {
            let args = AutogenArgs {
                course_name: c.course_name.clone(),
                course_id: c.course_id,
                chapter_id: c.chapter_id,
                chapter_name: c.chapter_name.clone(),
                topic_id: c.topic_id,
                topic_name: c.topic_name.clone(),
                count,
            };
            run_autogen(&None, prompt_path, &args, data_folder.as_str()).await;
            counter += 1;
            println!("Finished {} of {}", counter, len);
            println!("----------------------------------");
        }
    }

    if chapter_id.is_none() && course_id.is_none() {}
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
