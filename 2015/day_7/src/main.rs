mod file_processor_file;
mod component_traits;
mod components;
mod circuit;

use crate::circuit::circuit_board::CircuitBoard;
use crate::circuit::circuit_factory::CircuitFactory;
use crate::file_processor_file::FileProcessor;

fn main() {
    const INPUT: &str = "input.txt";
    let file_content: String;
    let mut board: CircuitBoard;


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

    board = CircuitFactory::create_circuit_from_file(&file_content);
    //board.print_all_wires();
    let answer1: u16 = board.resolve_board("a").unwrap();
    println!("The answer of a is: {:?}", answer1);

    board = CircuitFactory::create_circuit_from_file(&file_content); //clean board
    board.find_wire("b").unwrap().borrow_mut().set_value(answer1);
    
    let answer2: u16 = board.resolve_board("a").unwrap();
    println!("The answer of a is: {:?}", answer2);
}
