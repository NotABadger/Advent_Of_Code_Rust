mod file_processor_file;
mod replacement;
mod parse_text;
mod machine_calibration;
mod reverse_engineer_molecule;
mod reverse_engineer_molecule_attempt_two;
mod molecule_data;

use replacement::Replacement;

use crate::file_processor_file::FileProcessor;
use crate::machine_calibration::calibrate_machine;
//use crate::reverse_engineer_molecule::reverse_engineer_molecule;
//use crate::reverse_engineer_molecule_attempt_two::reverse_engineer_molecule;
use std::process::exit;

const FILE: &str = "input.txt";
fn main() {
    let file_content: String;
    let replacements: Vec<Replacement>;
    let molecule: String;
    // let mut replacements: Vec<Replacement> = Vec::new();
    // replacements.push(Replacement { find: "e".to_string(), replace: "H".to_string() });
    // replacements.push(Replacement { find: "e".to_string(), replace: "O".to_string() });
    // replacements.push(Replacement { find: "H".to_string(), replace: "HO".to_string() });
    // replacements.push(Replacement { find: "H".to_string(), replace: "OH".to_string() });
    // replacements.push(Replacement { find: "O".to_string(), replace: "HH".to_string() });
    // let molecule: String = "HOHOHO".to_string();

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

    //reverse_engineer_molecule::reverse_engineer_molecule(&molecule, &replacements);
    reverse_engineer_molecule_attempt_two::reverse_engineer_molecule(&molecule, &replacements);
}
