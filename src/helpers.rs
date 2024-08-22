use std::path::Path;

pub fn get_filename_without_extension(file_path: &str) -> Option<String> {
    Path::new(file_path)
        .file_stem() // Gets the file name without extension
        .and_then(|os_str| os_str.to_str()) // Converts it to &str
        .map(|s| s.to_string()) // Converts it to String
}
