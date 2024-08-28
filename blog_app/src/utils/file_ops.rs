pub struct FileInfo {
    pub name: String,
    pub path: String,
}

/// read all files of a given extension from a directory and all its subdirectories
/// then return a vector of each file name without its extension and its path
pub fn read_files_from_dir(dir: &str, ext: &str) -> Vec<FileInfo> {
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
                let name = path
                    .split('/')
                    .last()
                    .unwrap()
                    .split('.')
                    .next()
                    .unwrap()
                    .to_string();
                files.push(FileInfo {
                    name,
                    path: path.to_string(),
                });
            }
        }
    }
    files
}

pub fn read_file(file: &str) -> Result<String, String> {
    match std::fs::read_to_string(file) {
        Ok(c) => Ok(c),
        Err(e) => Err(format!("Error reading file: [{:?}]", e)),
    }
}
