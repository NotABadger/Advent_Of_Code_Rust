mod file_processor_file;
mod guests;
mod person;
mod relation;
mod guest_list_parser;
mod brute_force_all_seatings;

use file_processor_file::FileProcessor;
use guest_list_parser::parse_guest_list;
use guests::Guests;
use brute_force_all_seatings::resolve_seating_problem;

use std::rc::Rc;

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

    let guests : Rc<Guests> = Rc::new(parse_guest_list(&file));
    resolve_seating_problem(guests);
}
