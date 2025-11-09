use std::{env, fs, path::PathBuf};

// Read file names in directory
pub fn read_dir(path: &PathBuf) -> Result<Vec<String>, std::io::Error> {
    let mut result: Vec<String> = vec![];
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let path = entry.path();
        if let Some(name) = path.file_name() {
            if let Some(name) = name.to_str() {
                result.push(name.to_string());
            }
        }
    }
    Ok(result)
}

// Get PathBuf from &str
pub fn resolve_path(path: &str) -> Option<PathBuf> {
    let mut current_path = env::current_dir().unwrap();
    let parts: Vec<&str> = path.split("/").collect();
    for part in parts {
        current_path.push(part);
    }
    Some(current_path)
}
