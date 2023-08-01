mod file_processor_file;
mod component_traits;
mod wire;
mod components;
mod circuit;


use crate::file_processor_file::FileProcessor;
use crate::components::{*};
use crate::circuit::circuit_factory::CircuitFactory;

fn main() {
    const INPUT: &str = "input.txt";
    let file_content: String;

    if !FileProcessor::check_file_exists(INPUT)
    {
        panic!("Can't find input file!");
    }
    
    match FileProcessor::read_file(INPUT)
    {
        Ok(read_content) => file_content = read_content,
        Err(_msg) => {
            println!("Could not read file!");
            panic!();
        }
    }

    let mut factory : CircuitFactory = CircuitFactory::new();
    factory.create_circuit_from_file(&file_content);
}
