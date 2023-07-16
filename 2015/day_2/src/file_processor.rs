use std::fs; //File System
use std::path::Path;

pub struct FileProcessor
{

}

impl FileProcessor{

    pub fn read_file(file_path : &str) -> Result<String, &'static str>
    {
        let file_read_result = fs::read_to_string(file_path);
        match file_read_result {
            Ok(content) => return Ok(content),
            Err(_error) => return Err("Opening and reading failed."),
        };
    }

    pub fn check_file_exists(file_path : &str) -> bool
    {
        let file_check_return_val = Path::new(file_path).try_exists();
        match file_check_return_val {
            Ok(result) => return result,
            Err(_err) => return false,            
        };
    }
}