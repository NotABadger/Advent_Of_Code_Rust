mod tile;
mod directions;
mod part1;
mod error;

use input_file_lib::read_file_content;
use tile::TileType;
use part1::find_furthest_point;


fn main() {
    println!("Day_10 Program");
    let file_cont: String = read_file_content().expect("Program expects an input file as parameter");
    //y, x coördinate system
    let mut start_point: (usize, usize) = (0,0);
    let mut ground_scan: Vec<Vec<TileType>> = Vec::new();
    for line in file_cont.lines() {
        let line_trim = line.trim();
        let mut line_scan: Vec<TileType> = Vec::new();
        for field in line_trim.chars() {
           line_scan.push(TileType::from(field)); 
        }
        ground_scan.push(line_scan);
    }
    drop(file_cont); //20 KB less ram :D

    start_point.0 = ground_scan.iter()
                    .position(| line| line.contains(&TileType::StartPos)).expect("Every scan has a start pos");
    start_point.1 = ground_scan.get(start_point.0).unwrap()
                    .iter()
                    .position(|point| point == &TileType::StartPos).unwrap();
    //start point initialized.
    //scan in 2D vec available, and start point known
    _ = find_furthest_point(start_point, &ground_scan);

}
