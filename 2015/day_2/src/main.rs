// use std::env; //Calling Parameters

mod file_processor; //imports file
mod elf_box;

use crate::file_processor::FileProcessor; //imports object type
use crate::elf_box::ElfBox;

use std::env;

fn main() {
   const FILE_PATH: &str = "input.txt";
   let mut boxes: Vec<ElfBox> = Vec::new();
   let mut feet_of_paper_needed: u32 = 0;
   let mut feet_of_ribbon_needed: u32 = 0;

    //println!("Program parameters");
    // let args: Vec<String> = env::args().collect();
    // for arg in args
    // {
    //     println!("{:?}", arg);
    // }

    if !FileProcessor::check_file_exists(FILE_PATH)
    {
        println!("Could not find file: {:?}", FILE_PATH);
        panic!();
    }
    let file_content = FileProcessor::read_file(FILE_PATH).unwrap();

    for line in file_content.lines()
    {
         match ElfBox::elf_box_from_string(line){
            Ok(box) => boxes.push(box),
            Error(err) => println!("Parsing box failed, msg{:?}", err),
         }
    }

    for mut individual_box in boxes
    {
        individual_box.calculate_areas();
        feet_of_paper_needed += individual_box.calculate_feet_of_paper_required();
        feet_of_ribbon_needed += individual_box.calculate_feet_of_ribbon_required();
    }

    println!("Total square feet of paper we need to order is: {:?}", feet_of_paper_needed);
    println!("Total length in feet of ribon we need to order is: {:?}", feet_of_ribbon_needed);
}
