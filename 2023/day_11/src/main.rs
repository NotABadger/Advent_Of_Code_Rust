mod universe;
mod part1;
mod part2;
use input_file_lib::read_file_content;
use universe::Universe;
use part1::part1;
use part2::part2;

fn main() {
    println!("Day 11 program");
    let file_content = read_file_content().expect("Expected input file as first parameter");
    let universe: Universe = Universe::from_file_content(&file_content);
    println!("{}",universe.to_string());
    part1( universe.clone());
    part2(universe);
}
