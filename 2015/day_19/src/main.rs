mod file_processor_file;
mod replacement;
mod parse_text;

use replacement::Replacement;

use crate::file_processor_file::FileProcessor;
use std::process::exit;

const FILE: &str = "input.txt";
fn main() {
    let file_content: String;
    let mut replacements: Vec<Replacement> = Vec::new();
   

    if !FileProcessor::check_file_exists(FILE)
    {
        println!("Sorry, but couldn't find the input file");
        exit(0);
    }
    match FileProcessor::read_file(FILE) {
        Ok(read_content) => file_content = read_content,
        Err(msg) => panic!("{}",msg),
    }
   


}
