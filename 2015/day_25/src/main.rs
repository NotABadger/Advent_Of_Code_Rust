mod launch_code;

use launch_code::LaunchCode;


const REQUIRED_ROW: u32 = 2947;
const REQUIRED_COLUMN: u32 = 3029;

fn main() {
    println!("day_25");
    let mut code_list: Vec<LaunchCode> = Vec::with_capacity((REQUIRED_COLUMN * REQUIRED_ROW) as usize);
    code_list.push(LaunchCode::new(1, 1, 20151125));
    let mut highest_row: u32 = 1;

    let mut current_row: u32 = 1;
    let mut current_column: u32 = 1;
    while current_row != REQUIRED_ROW || 
            current_column != REQUIRED_COLUMN {
        let next_code = LaunchCode::generate_from_previous(code_list.last().unwrap(), &mut highest_row);
        current_column = next_code.get_column();
        current_row = next_code.get_row();
        code_list.push(next_code);
        //println!("generated row: {}, column: {}", current_row, current_column);
    }

    
    let launch_code_ref: &LaunchCode = code_list.iter().find(|&code| code.get_row() == REQUIRED_ROW && code.get_column() == REQUIRED_COLUMN).unwrap();
    println!("row: {} column: {}, code: {}", launch_code_ref.get_row(), launch_code_ref.get_column(), launch_code_ref.get_code());
    
}
