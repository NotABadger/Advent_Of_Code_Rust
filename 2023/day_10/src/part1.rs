//part 1, question, find loop & furthest point away from starting point.

use crate::tile::TileType;
use crate::tile::PipeType;
use crate::directions::Directions;

pub fn find_furthest_point(start_point: (usize, usize), scan: &Vec<Vec<TileType>>) -> usize {

    let mut max_steps: usize = 0;
    if direction_available(start_point, start_point, scan, Directions::North, &mut max_steps) {
        println!("Found a path heading {:?}, and took me {} distance from the point", Directions::North, max_steps);
    }
    max_steps = 0;
    if direction_available(start_point, start_point,scan, Directions::South, &mut max_steps) {
        println!("Found a path heading {:?}, and took me {} distance from the point", Directions::South, max_steps);
    }
    max_steps = 0;
    if direction_available(start_point, start_point, scan, Directions::East, &mut max_steps) {
        println!("Found a path heading {:?}, and took me {} distance from the point", Directions::East, max_steps);
    }
    max_steps = 0;
    if direction_available(start_point, start_point, scan, Directions::West, &mut max_steps) {
        println!("Found a path heading {:?}, and took me {} distance from the point", Directions::West, max_steps);
    }
    
    return 0;
}


fn direction_available(start_point: (usize, usize), current_point: (usize, usize), scan: &Vec<Vec<TileType>>, direction: Directions, max_steps: &mut usize) -> bool {
    let (mut current_y, mut current_x) = current_point;
    
    //are we heading off map?
    match direction {
        Directions::North => if current_y > 0 {current_y -= 1} else {return false},
        Directions::South => if current_y < scan.len()-1 {current_y += 1 } else {return false},
        Directions::East => if current_x < scan.len()-1 {current_x += 1 } else {return false},
        Directions::West => if current_x > 0 {current_x -= 1} else {return false},
    }
    //is the pipe ending?
    let next_pipe = scan.get(current_y).expect("we checked bounds")
                                    .get(current_x).expect("we checked bounds");
    match *next_pipe {
        TileType::Ground => return false, //pipe ended
        TileType::StartPos => return true, //back at start
        _ => () //check directions next
    }
    //Checking if we run into a wall
    match direction {
        Directions::North => if *next_pipe == TileType::Pipe(PipeType::Horizontal) || 
                                 *next_pipe == TileType::Pipe(PipeType::CornerNE) || 
                                 *next_pipe == TileType::Pipe(PipeType::CornerNW) 
                                 { return false},
        Directions::South => if *next_pipe == TileType::Pipe(PipeType::Horizontal) || 
                                *next_pipe == TileType::Pipe(PipeType::CornerSE) || 
                                *next_pipe == TileType::Pipe(PipeType::CornerSW) 
                                { return false},
        Directions::East => if *next_pipe == TileType::Pipe(PipeType::Vertical) || 
                                *next_pipe == TileType::Pipe(PipeType::CornerSE) || 
                                *next_pipe == TileType::Pipe(PipeType::CornerNE) 
                                { return false},
        Directions::West => if *next_pipe == TileType::Pipe(PipeType::Vertical) || 
                                *next_pipe == TileType::Pipe(PipeType::CornerSW) || 
                                *next_pipe == TileType::Pipe(PipeType::CornerNW) 
                                { return false},
    }

    //We are okay to travel!
    //determine new position
    let mut distance = current_y.abs_diff(start_point.0);
    distance += current_x.abs_diff(start_point.1);
    if distance > *max_steps {
        *max_steps = distance;
    }
    //determine new direction
    let mut next_direction = Directions::North;
    if let TileType::Pipe(pipe) = next_pipe {
        next_direction = pipe.determine_outgoing_direction(direction).unwrap();
    }
    else {
        panic!("All other pipe types should have been covered before!");
    }
    dbg!(&direction);
    dbg!(&next_direction);
    dbg!(&distance);
    dbg!(&current_point);
    dbg!(next_pipe);


    //might run into dead end
    direction_available(start_point, (current_y, current_x), scan, next_direction, max_steps)
}

