mod light;
mod file_processor_file;
mod instruction;
mod instruction_parser;
mod light_controller;

use crate::instruction::instruction_mod::Instruction;
use crate::file_processor_file::file_processor::{*};
use crate::instruction_parser::instruction_parser_mod::{*};
use crate::light_controller::light_controller::execute_light_commands;

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

    let instructions: Box<Vec<Instruction>> = parse_file_to_list_instructions(&file_content);
    drop(file_content);
    println!("Only the execution part to go...");
    let amount_of_lights_on: u32 = execute_light_commands(instructions);// = execute_light_commands(instructions);
    //println!("Amount of lights turned on: {:?}", amount_of_lights_on);
    println!("Total brightness of the lights: {:?}", amount_of_lights_on);

}
