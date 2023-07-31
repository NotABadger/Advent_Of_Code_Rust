mod file_processor_file;
mod component_traits;
mod wire;
mod components;
mod circuit;


use crate::file_processor_file::FileProcessor;
use crate::components::{*};
fn main() {
    const INPUT: &str = "input.txt";
    let _file_content: String;

    if !FileProcessor::check_file_exists(INPUT)
    {
        panic!("Can't find input file!");
    }
    
    match FileProcessor::read_file(INPUT)
    {
        Ok(read_content) => _file_content = read_content,
        Err(_msg) => {
            println!("Could not read file!");
            panic!();
        }
    }


}
