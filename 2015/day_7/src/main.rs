mod file_processor_file;
mod component_traits;
mod wire;
mod components;
mod circuit;


use crate::file_processor_file::FileProcessor;
use crate::circuit::circuit_factory::CircuitFactory;
use crate::circuit::circuit_board::CircuitBoard;
use crate::circuit::circuit_resolver::CircuitResolver;

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
    let mut complete_board: CircuitBoard = CircuitFactory::create_circuit_from_file(&file_content);
    println!("The circuit contained {:?} wires and {:?} components", complete_board.get_mut_connections().len(), complete_board.get_mut_connections().len());
    
    let final_answer : u16 = CircuitResolver::resolve_circuit(&mut complete_board, "a");
    println!("The value of wire \"a\" is: {:?}", final_answer);
}
