mod file_processor_file;
mod lightgrid;
mod light;
mod text_to_grid;
mod grid_processor;

use crate::file_processor_file::FileProcessor;
use crate::text_to_grid::text_to_grid;
use crate::lightgrid::LightGrid;
use crate::grid_processor::process_next_frame;
use std::process::exit;

const FILE: &str = "input.txt";
const AMOUNT_OF_ITTERATIONS: u32 = 100;
fn main() {
    let file_content: String;
    let light_grid: LightGrid;

    if !FileProcessor::check_file_exists(FILE)
    {
        println!("Sorry, but couldn't find the input file");
        exit(0);
    }
    match FileProcessor::read_file(FILE) {
        Ok(read_content) => file_content = read_content,
        Err(msg) => panic!("{}",msg),
    }
    light_grid = text_to_grid(&file_content);

    let mut working_grid:LightGrid = light_grid.clone();
    // part two snippet, stuck corners \\
    let len_line: usize = working_grid.grid.len();
    let len_column: usize = working_grid.grid[0].len();
    working_grid.grid[0][0].state = true;
    working_grid.grid[0][len_column-1].state = true;
    working_grid.grid[len_line-1][0].state = true;
    working_grid.grid[len_line-1][len_column-1].state = true;
    // end part two snippet, stuck corners \\

    for _index in 0..AMOUNT_OF_ITTERATIONS
    {
        working_grid = process_next_frame(&mut working_grid);
        // part two snippet, stuck corners \\
        working_grid.grid[0][0].state = true;
        working_grid.grid[0][len_column-1].state = true;
        working_grid.grid[len_line-1][0].state = true;
        working_grid.grid[len_line-1][len_column-1].state = true;
        // end part two snippet, stuck corners \\
    }

    println!("{}",working_grid.lights_currently_on());
}
