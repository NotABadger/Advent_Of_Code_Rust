/* The problem given here is "determine if a point is inside a 2D Polygon" 
    This can be solved with "ray tracing" and counting the amount the line tracing the outline of the polygon is crossed.
    Odd numbers means that a non-line block is inside the Polygon, and even nummbers means that the non-line block is inside the polygon */

mod tile_part2;
mod pipe_tracer;
use tile_part2::{PipeType, Tile};
use crate::tile::TileType;
use pipe_tracer::trace_pipe_route;

pub fn part2(start_point: (usize, usize), scan: &Vec<Vec<TileType>>) { 
    //step one, trace the pipes through the maze & mark these
    let mut marked_map = trace_pipe_route(start_point, scan);
    //Step two, trace on horizontal or vertical the inner blocks & count
    let amount_inside_shape = mark_and_count_inside_polygon(&mut marked_map);
    println!("found {} spaces inside the shape", amount_inside_shape);
}

fn mark_and_count_inside_polygon(traced_scan : &mut Vec<Vec<Tile>>) -> usize
{
    /*
    ...........
    .S-------7.
    .|F-----7|.
    .||OOOOO||.
    .||OOOOO||.
    .|L-7OF-J|.
    .|II|O|II|.
    .L--JOL--J.
    .....O.....
    Example, I is inside, O is outside.
    Whenever we hit a vertical line, we definitally switch inside/outside
    Horizontal lines don't change a thing (we will get another traveled square anyway)
    corners though... 
    S------7X -> X is outside
    |      |
---------------
           |
    S------JX -> X is inside
    |      
 */
    let mut amount_inside: usize = 0;
    for line in traced_scan.iter_mut() {
        let mut inside_shape : bool = false;
        let mut on_border_of_line: bool = false;
        let mut entered_from_below: bool = false;
        for tile in line.iter_mut() {
            if tile.get_traveled() {
                match tile.get_tile_type() {
                    TileType::Pipe(PipeType::Vertical) => { inside_shape = !inside_shape; },
                    TileType::Pipe(PipeType::Horizontal) => (),
                    TileType::Pipe(PipeType::CornerNE) => { //Scanning left to right, so we enter
                        if !on_border_of_line {
                            //first corner to find
                            on_border_of_line = true;
                            entered_from_below = false; //Comming from north or east
                        }},
                    TileType::Pipe(PipeType::CornerSE) => { 
                        if !on_border_of_line {
                        //first corner to find
                        on_border_of_line = true;
                        entered_from_below = true; //Comming from South or east
                    }},
                    TileType::Pipe(PipeType::CornerNW) => { 
                        if on_border_of_line {
                            on_border_of_line = false;
                            if entered_from_below {
                                inside_shape = !inside_shape;
                            }
                        }
                    },
                    TileType::Pipe(PipeType::CornerSW) => { 
                        if on_border_of_line {
                            on_border_of_line = false;
                            if !entered_from_below {
                                inside_shape = !inside_shape;
                            }
                        }
                    },
                    TileType::StartPos => {
                        //HARD CODED, I known StartPos is a SW corner(see output part 1)!
                        if on_border_of_line {
                            on_border_of_line = false;
                            if !entered_from_below {
                                inside_shape = !inside_shape;
                            }
                        }
                    },
                    TileType::Ground => (),
                }
            }
            else {
                if inside_shape {
                    tile.set_inside_shape();
                    amount_inside +=1;
                }
            }
        }
    }  
    amount_inside  
}