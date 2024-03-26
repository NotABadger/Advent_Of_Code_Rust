mod file_processor_file;
mod package_group;
mod first_group_finder;

use file_processor_file::FileProcessor;
use first_group_finder::*;
use std::process::exit;


const FILE: &str = "input.txt";

fn main() {
    println!("Day_24 challange");

    let file_content: String;
    let numbers: Vec<i32>;
    let third_of_tot_val: i32;

    if !FileProcessor::check_file_exists(FILE)
    {
        println!("Sorry, but couldn't find the input file");
        exit(0);
    }
    match FileProcessor::read_file(FILE) {
        Ok(read_content) => file_content = read_content,
        Err(msg) => panic!("{}",msg),
    }

    numbers = parse_file_content(&file_content);
    let mut total_value: i32 = 0;
    numbers.iter().for_each(| int| total_value += *int);
    third_of_tot_val = total_value / 3;

    println!("The third of total val is: {}", third_of_tot_val);
    let groups: Vec<PackageGroup> = find_first_combo(&numbers, third_of_tot_val);
    
    
}

fn parse_file_content(file_content: &str) -> Vec<i32>
{
    let mut ret_val: Vec<i32> = Vec::new();
    let file_trimmed = file_content.trim();

    for line in file_trimmed.lines()
    {
        let line_trimmed: &str = line.trim();
        let number: i32 = line_trimmed.parse::<i32>().unwrap();
        ret_val.push(number);
    }

    ret_val
}