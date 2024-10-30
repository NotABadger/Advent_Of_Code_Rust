//part 1, question, find loop & furthest point away from starting point.

use crate::tile::TileType;
use crate::tile::PipeType;
use crate::directions::Directions;

pub fn find_furthest_point(start_point: (usize, usize), scan: &Vec<Vec<TileType>>) {

    let mut max_steps: usize = 0;
    let mut next_dir:Directions = Directions::North;
    let mut found_route: bool = false;
    let mut current_point: (usize, usize) = start_point;
    for _ in 0..scan.len()*scan.get(0).unwrap().len() {
        if !travel_route(start_point, &mut current_point, scan, next_dir, &mut next_dir, &mut max_steps, &mut found_route) {
            break;
        }
        
    }
    if found_route {
        println!("Found route going North, max distance was {}", max_steps /2);
    }
    else {
        println!("North was a dead end"); 
    }

    max_steps = 0;
    next_dir = Directions::South;
    found_route = false;
    current_point = start_point;
    for _ in 0..scan.len()*scan.get(0).unwrap().len() {
        if !travel_route(start_point, &mut current_point, scan, next_dir, &mut next_dir, &mut max_steps, &mut found_route) {  
            break;
        }//travelin'
    }
    if found_route {
        println!("Found route going South, max distance was {}",( max_steps /2) +1);
    }
    else {
        println!("South was a dead end"); 
    }
    max_steps = 0;
    next_dir = Directions::East;
    found_route = false;
    current_point = start_point;
    for _ in 0..scan.len()*scan.get(0).unwrap().len() {
        if !travel_route(start_point, &mut current_point, scan, next_dir, &mut next_dir, &mut max_steps, &mut found_route) {
            break;
        }
        //travelin'
    }
    if found_route {
        println!("Found route going East, max distance was {}", ( max_steps /2) +1);
    }
    else {
        println!("East was a dead end"); 
    }
    max_steps = 0;
    next_dir = Directions::West;
    found_route = false;
    current_point = start_point;
    for _ in 0..scan.len()*scan.get(0).unwrap().len() {
        if !travel_route(start_point, &mut current_point, scan, next_dir, &mut next_dir, &mut max_steps, &mut found_route) {
            break;
        }
        //travelin'
    }
    if found_route {
        println!("Found route going West, max distance was {}", ( max_steps /2) +1);
    }
    else {
        println!("West was a dead end"); 
    }
}


fn travel_route(start_point: (usize, usize), current_point: &mut (usize, usize), scan: &Vec<Vec<TileType>>, 
                        direction: Directions, next_direction: &mut Directions, max_steps: &mut usize, 
                        found_route: &mut bool) -> bool {
    let (mut current_y, mut current_x) = current_point;
    *found_route = false;
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
        TileType::StartPos =>{ *found_route = true;
                                 return true}, //back at start
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
    *current_point = (current_y, current_x);
    *max_steps += 1;
    //determine new direction
    if let TileType::Pipe(pipe) = next_pipe {
        *next_direction = pipe.determine_outgoing_direction(direction).unwrap();
    }
    else {
        panic!("All other pipe types should have been covered before!");
    }
    // dbg!(&direction);
    // dbg!(&next_direction);
    // dbg!(&distance);
    // dbg!(&current_point);
    // dbg!(next_pipe);


    //Would love to do recursion, but stack overflow will happen...
    true
}

