//Question: springs positioned in rows, row has springs in three states, working, broken, unknown.
//each row has arrangement of broken springs,
// how can we map the arrangement of springs so it matches the given arrangement of broken springs

//Here I will implement the algorithm which options there are.
//This is a recursive call, we will try the first element where it fits, then the second etc.
//When it succeeded (or failed), we will return and try shifting it to the right. 

//The order of elements is very important, and mapping on the original string.
// ???.### 1,1,3 -> The '3' is broken... 

use crate::spring_state::SpringState;
use crate::SpringsRow;

pub fn check_possible_solutions(spring_row: &SpringsRow) -> u32 {

    let mut working_row = spring_row.get_row_spring_states().clone();
    try_broken_position(0, 0, &mut working_row, spring_row.get_arrangment_order())
}

fn try_broken_position(index_broken_spring: usize, index_startpoint_row: usize, working_row: &mut Vec<SpringState>, broken_spring_pattern: &Vec<u8>) -> u32 {
    let mut found_combinations: u32 = 0;
    let mut function_working_row = working_row.clone();
    //Let's see if we can fit at the starting place, or later.
    //make sure we can fit 'amount of broken springs' in this place or rest of the string.
    let amount_of_broken_springs = broken_spring_pattern.get(index_broken_spring).unwrap();




    
    found_combinations
}