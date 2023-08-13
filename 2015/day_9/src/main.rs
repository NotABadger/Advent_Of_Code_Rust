mod file_processor_file;
mod city;
mod city_parser;

use file_processor_file::FileProcessor;
use city::City;
use city_parser::parse_cities;


fn main() {
    const INPUT: &str = "input.txt";
    let cities_list: Vec<City>;

    if !FileProcessor::check_file_exists(INPUT)
    {
        panic!("Can't find input file!");
    }
    
    match FileProcessor::read_file(INPUT)
    {
        Ok(read_content) => cities_list = parse_cities(&read_content),
        Err(_msg) => {
            println!("Could not read file!");
            panic!();
        }
    }

    for city in cities_list
    {
        println!("{:?}", city);
    }
}
