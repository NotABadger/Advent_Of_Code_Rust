mod container;
mod file_processor_file;
mod container_parser;
mod combo_calculater;
use std::process::exit;

use container::*;
use file_processor_file::FileProcessor;

use crate::container_parser::text_to_containers;


const FILE: &str = "input.txt";
fn main() {
    let file_content: String;
    let mut containers: Vec<ContainerType>;
    let volume: u32 = 150;
    
    if !FileProcessor::check_file_exists(FILE)
    {
        println!("Sorry, but couldn't find the input file");
        exit(0);
    }
    match FileProcessor::read_file(FILE) {
        Ok(read_content) => file_content = read_content,
        Err(msg) => panic!("{}",msg),
    }
    containers = text_to_containers(&file_content);
    containers.sort_by(|a, b| b.capacity.cmp(&a.capacity));

    println!("{} combinations are possible with {} volume", combo_calculater::check_combos_with_largest_container(&containers, volume), volume);
}
