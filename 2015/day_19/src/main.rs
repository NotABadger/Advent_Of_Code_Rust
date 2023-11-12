mod file_processor_file;
mod replacement;
mod parse_text;
mod machine_calibration;

use replacement::Replacement;

use crate::file_processor_file::FileProcessor;
use crate::machine_calibration::calibrate_machine;
use std::process::exit;

const FILE: &str = "input.txt";
fn main() {
    let file_content: String;
    let replacements: Vec<Replacement>;
    let molecule: String;

    if !FileProcessor::check_file_exists(FILE)
    {
        println!("Sorry, but couldn't find the input file");
        exit(0);
    }
    match FileProcessor::read_file(FILE) {
        Ok(read_content) => file_content = read_content,
        Err(msg) => panic!("{}",msg),
    }
   
    replacements = parse_text::parse_replacements(&file_content);
    molecule = parse_text::parse_molecule(&file_content);
    
    let mut found_mutations: Vec<String> = Vec::new();
    calibrate_machine(&molecule, &replacements, &mut found_mutations);

    println!("Amount of mutations found: {}", found_mutations.len());
}
