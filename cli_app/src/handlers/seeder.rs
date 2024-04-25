use std::path::PathBuf;

use sqlx::MySqlPool;

pub async fn seeder(courses_file: &Option<PathBuf>, chapters_file: &Option<PathBuf>, topics_file: &Option<PathBuf>, pool: &MySqlPool) {
    if courses_file.is_some() {
        println!("Processing courses file");
        // verify the file exists
        // validate the file against the schema
        // read courses from the file
        // save courses to the database
    } else if chapters_file.is_some() {
        println!("Processing chapters file");
        // verify the file exists
        // validate the file against the schema
        // read chapters from the file
        // save chapters to the database
    } else if topics_file.is_some() {
        println!("Processing topics file");
        // verify the file exists
        // validate the file against the schema
        // read topics from the file
        // save topics to the database
    }
}
