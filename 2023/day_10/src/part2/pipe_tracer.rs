use std::usize;

use crate::tile::TileType;
use crate::tile::PipeType;
use crate::directions::Directions;

use super::tile_part2::Tile;

pub fn trace_pipe_route((start_y, start_x): (usize, usize), original_scan: &Vec<Vec<TileType>>) -> Vec<Vec<Tile>> {
    //copy original scan
    let mut traced_scan: Vec<Vec<Tile>> = copy_original_scan(original_scan);
    //leave in all 4 directions
    if try_direction(Directions::North, (start_y, start_x), &mut traced_scan) {
        println!("Found {:?} route!", Directions::North );
        return traced_scan;
    }
    if try_direction(Directions::South, (start_y, start_x), &mut traced_scan) {
        println!("Found {:?} route!", Directions::South );
        return traced_scan;
    }
    if try_direction(Directions::East, (start_y, start_x), &mut traced_scan) {
        println!("Found {:?} route!", Directions::East );
        return traced_scan;
    }
    if try_direction(Directions::West, (start_y, start_x), &mut traced_scan) {
        println!("Found {:?} route!", Directions::West );
        return traced_scan;
    }
    panic!("No routes were found in part 2!");
}

fn copy_original_scan(original_scan: &Vec<Vec<TileType>>) -> Vec<Vec<Tile>>
{
    let mut new_scan_data: Vec<Vec<Tile>> = Vec::with_capacity(original_scan.len());
    for line in original_scan {
        let mut new_scan_line: Vec<Tile> = Vec::new();
        for tile in line {
            new_scan_line.push(Tile::from_tile_type(tile));
        }
        new_scan_data.push(new_scan_line);
    }
    new_scan_data
}

fn try_direction(direction_to_start: Directions, (start_y, start_x): (usize, usize), original_scan: &mut Vec<Vec<Tile>>) -> bool {
    let mut current_direction: Directions = direction_to_start;
    let (mut current_y, mut current_x) = (start_y, start_x);
    let mut valid_route: bool = false;
    while can_travel_to_next_pipe(original_scan, current_y, current_x, current_direction, &mut valid_route)
    {
        move_cordinates(original_scan, &mut current_y, &mut current_x, &mut current_direction);
    }
    if valid_route {
        //only update map when traveled route is found
        current_direction = direction_to_start;
        current_y = start_y;
        current_x = start_x;
        while can_travel_to_next_pipe(original_scan, current_y, current_x, current_direction, &mut valid_route)
        {
            mark_current_pipe( original_scan, current_y, current_x);
            move_cordinates(original_scan, &mut current_y, &mut current_x, &mut current_direction);
        }
        mark_current_pipe( original_scan, current_y, current_x);
    }
    valid_route
}

fn can_travel_to_next_pipe(map: &Vec<Vec<Tile>>, current_y: usize, current_x: usize, current_direction: Directions, valid_route :&mut bool) -> bool {
    //are we heading off map?
    let mut next_y = current_y;
    let mut next_x = current_x;
    match current_direction {
        Directions::North => if next_y > 0 {next_y -= 1} else {return false},
        Directions::South => if next_y < map.len()-1 {next_y += 1 } else {return false},
        Directions::East => if next_x < map.get(1).unwrap().len()-1 {next_x += 1 } else {return false},
        Directions::West => if next_x > 0 {next_x -= 1} else {return false},
    }
    //is the pipe ending?
    let next_tile = map.get(next_y).expect("we checked bounds")
                                    .get(next_x).expect("we checked bounds");
    match next_tile.get_tile_type() {
        TileType::Ground => return false, //pipe ended
        TileType::StartPos =>{ *valid_route = true;
                                 return false}, //back at start
        _ => () //check directions next
    }
    //Checking if we run into a wall
    let next_type = next_tile.get_tile_type();
    match current_direction {
        Directions::North => if next_type == TileType::Pipe(PipeType::Horizontal) || 
                                next_type == TileType::Pipe(PipeType::CornerNE) || 
                                next_type == TileType::Pipe(PipeType::CornerNW) 
                                 { return false},
        Directions::South => if next_type == TileType::Pipe(PipeType::Horizontal) || 
                                next_type == TileType::Pipe(PipeType::CornerSE) || 
                                next_type == TileType::Pipe(PipeType::CornerSW) 
                                { return false},
        Directions::East => if next_type == TileType::Pipe(PipeType::Vertical) || 
                                next_type == TileType::Pipe(PipeType::CornerSE) || 
                                next_type == TileType::Pipe(PipeType::CornerNE) 
                                { return false},
        Directions::West => if next_type == TileType::Pipe(PipeType::Vertical) || 
                                next_type == TileType::Pipe(PipeType::CornerSW) || 
                                next_type == TileType::Pipe(PipeType::CornerNW) 
                                { return false},
    }
    true
}

fn move_cordinates(map: &Vec<Vec<Tile>>, current_y: &mut usize, current_x: &mut usize, current_direction: &mut Directions) {
    //we know we can travel
    match current_direction {
        Directions::North => *current_y -= 1,
        Directions::South => *current_y += 1,
        Directions::East => *current_x += 1,
        Directions::West => *current_x -= 1,
    }
    let next_tile = map.get(*current_y).unwrap().get(*current_x).unwrap();
    if let TileType::Pipe(pipe) = next_tile.get_tile_type() {
        *current_direction = pipe.determine_outgoing_direction(*current_direction).unwrap();
    }
    else {
        panic!("Can_travel_to_next_pipe() should have covered it all");
    }
}

fn mark_current_pipe(map: &mut Vec<Vec<Tile>>, current_y: usize, current_x: usize) {
    let tile: &mut Tile = map.get_mut(current_y).unwrap().get_mut(current_x).unwrap();
    tile.set_traveled();
}