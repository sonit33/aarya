use std::{ fs::File, io::{ Read, Write } };

#[derive(Debug)]
pub enum FileOpsErrorTypes {
    FileOpenError(String, String),
    FileReadError(String, String),
    FileCreateError(String, String),
    FileWriteError(String, String),
}
#[derive(Debug)]
pub enum FileOpsResult {
    Success(String),
    Error(FileOpsErrorTypes),
}

/// check if file exists
pub fn file_exists(path: &str) -> bool {
    std::path::Path::new(path).exists()
}

// read file contents
pub fn read_file_contents(path: &str) -> FileOpsResult {
    match File::open(path) {
        Ok(mut file) => {
            let mut contents = String::new();
            match file.read_to_string(&mut contents) {
                Ok(_) => FileOpsResult::Success(contents),
                Err(e) =>
                    FileOpsResult::Error(
                        FileOpsErrorTypes::FileReadError(format!("{}", e), path.to_string())
                    ),
            }
        }
        Err(e) =>
            FileOpsResult::Error(
                FileOpsErrorTypes::FileOpenError(format!("{}", e), path.to_string())
            ),
    }
}

// write to file
pub fn write_to_file(path: &str, contents: &str) -> FileOpsResult {
    match File::create(path) {
        Ok(mut file) =>
            match file.write_all(contents.as_bytes()) {
                Ok(_) => FileOpsResult::Success("File written successfully".to_string()),
                Err(e) =>
                    FileOpsResult::Error(
                        FileOpsErrorTypes::FileWriteError(format!("{}", e), path.to_string())
                    ),
            }
        Err(e) =>
            FileOpsResult::Error(
                FileOpsErrorTypes::FileCreateError(format!("{}", e), path.to_string())
            ),
    }
}
