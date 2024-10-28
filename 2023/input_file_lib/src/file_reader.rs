use std::fs; //File System
use std::path::Path;

pub fn read_file(file_path : &str) -> Result<String, &'static str>
{
    let file_read_result = fs::read_to_string(file_path);
    match file_read_result {
        Ok(content) => Ok(content.trim().to_string()),
        Err(_error) => Err("Opening and reading failed."),
    }
}

pub fn check_file_exists(file_path : &str) -> bool
{
    let file_check_return_val = Path::new(file_path).try_exists();
    file_check_return_val.unwrap_or_default()
}