mod file_processor_file;

use file_processor_file::FileProcessor;

fn main() {
    const INPUT: &str = "input.txt";
    let file : String;

    if !FileProcessor::check_file_exists(INPUT)
    {
        panic!("Can't find input file!");
    }
    
    match FileProcessor::read_file(INPUT)
    {
        Ok(read_content) => file = read_content,
        Err(_msg) => {
            println!("Could not read file!");
            panic!();
        }
    }
}
