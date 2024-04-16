use std::env;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

pub(crate) fn create_temp_file(file_name: &str, content: &str) -> PathBuf {
    let mut file_path = env::temp_dir();
    file_path.push(file_name);
    let mut file = File::create(&file_path).unwrap();
    file.write_all(content.as_bytes()).unwrap();
    while file.flush().is_err() {}
    file_path
}