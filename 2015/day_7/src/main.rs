mod Components;
mod file_processor_file;
mod component_traits;


use crate::file_processor_file::file_processor::{*};
fn main() {
    const INPUT: &str = "input.txt";
    let file_content: String;

    if !check_file_exists(INPUT)
    {
        panic!("Can't find input file!");
    }
    
    match read_file(INPUT)
    {
        Ok(read_content) => file_content = read_content,
        Err(_msg) => {
            println!("Could not read file!");
            panic!();
        }
    }


}
