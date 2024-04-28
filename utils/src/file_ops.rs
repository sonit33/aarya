use std::{
    fs::{self, File},
    io::{Read, Write},
};

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

pub struct FileInfo {
    pub name: String,
    pub path: String,
}

/// read all files of a given extension from a directory and all its subdirectories
/// then return a vector of each file name without its extension and its path
pub fn read_files_from_dir(
    dir: &str,
    ext: &str,
) -> Vec<FileInfo> {
    let mut files = Vec::new();
    let paths = std::fs::read_dir(dir).unwrap();
    for path in paths {
        let path = path.unwrap().path();
        if path.is_dir() {
            let mut sub_files = read_files_from_dir(path.to_str().unwrap(), ext);
            files.append(&mut sub_files);
        } else {
            let path = path.to_str().unwrap();
            if path.ends_with(ext) {
                let name = path.split('/').last().unwrap().split('.').next().unwrap().to_string();
                files.push(FileInfo { name, path: path.to_string() });
            }
        }
    }
    files
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
                Err(e) => FileOpsResult::Error(FileOpsErrorTypes::FileReadError(format!("{}", e), path.to_string())),
            }
        }
        Err(e) => FileOpsResult::Error(FileOpsErrorTypes::FileOpenError(format!("{}", e), path.to_string())),
    }
}

// write to file
pub fn write_to_file(
    path: &str,
    contents: &str,
) -> FileOpsResult {
    match File::create(path) {
        Ok(mut file) => match file.write_all(contents.as_bytes()) {
            Ok(_) => FileOpsResult::Success("File written successfully".to_string()),
            Err(e) => FileOpsResult::Error(FileOpsErrorTypes::FileWriteError(format!("{}", e), path.to_string())),
        },
        Err(e) => FileOpsResult::Error(FileOpsErrorTypes::FileCreateError(format!("{}", e), path.to_string())),
    }
}

pub fn make_dir(path: &str) -> FileOpsResult {
    match fs::create_dir_all(path) {
        Ok(_) => FileOpsResult::Success("Directory created successfully".to_string()),
        Err(e) => FileOpsResult::Error(FileOpsErrorTypes::FileCreateError(format!("{}", e), path.to_string())),
    }
}
