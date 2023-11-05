mod file_processor_file;
mod aunt;
mod aunt_property_parser;
use std::process::exit;

use file_processor_file::FileProcessor;
use aunt::Aunt;

const FILE: &str = "input.txt";

fn main() {
    let file_content: String;

    if !FileProcessor::check_file_exists(FILE)
    {
        println!("Sorry, but couldn't find the input file");
        exit(0);
    }
    match FileProcessor::read_file(FILE) {
        Ok(read_content) => file_content = read_content,
        Err(msg) => panic!("{}",msg),
    }

    let aunts_list: Vec<Aunt> = aunt_property_parser::parse_aunts(&file_content);
    let mut possible_senders: Vec<Aunt> = Vec::new();
    let mut aunt_sender = Aunt::default();
    aunt_sender.set_parameter("children", 3);
    aunt_sender.set_parameter("cats", 7);
    aunt_sender.set_parameter("samoyeds", 2);
    aunt_sender.set_parameter("pomeranians", 3);
    aunt_sender.set_parameter("akitas", 0);
    aunt_sender.set_parameter("vizslas", 0);
    aunt_sender.set_parameter("goldfish", 5);
    aunt_sender.set_parameter("trees", 3);
    aunt_sender.set_parameter("cars", 2);
    aunt_sender.set_parameter("perfumes", 1);

    for aunt in &aunts_list
    {
        if aunt_sender.is_possible_same(aunt)
        {
            possible_senders.push(aunt.clone());
        }
    }

    println!("There are {} possible aunts.", possible_senders.len());
    if possible_senders.len() > 0
    {
        let aunt_number = possible_senders.get(0).unwrap().aunt_nr;
        let number = aunt_number.expect("Found an aunt, and all aunts should have numbers");
        println!("the first matching aunt is {}",number);
    }

}
