mod springs_row;
mod spring_state;

use std::error::Error;

use input_file_lib::read_file_content;
use springs_row::SpringsRow;
use error_type_lib::MyError;

fn main() -> Result<(), Box<dyn Error>> {
    println!("day_12 program");
    let file_content: String = read_file_content()?;
    let spring_rows: Vec<SpringsRow> = parse_file_to_list(&file_content)?;
    //Algorithm:
    // only thing to modify are unknowns states/springs
    // Get current row Check first arrangment
    //      In recursion
    // Variables: unknown_state_index, arrangment_index, amount_of_successfull_combinations
    // 
    // Change unknown state to spring on index 0 (0 of unkown positions)
    // Check if this "improves" -> the arrangement order number goes up. (not over)
    // if goes over, put 
    Ok(())
}

fn parse_file_to_list(file_content: &str) -> Result<Vec<SpringsRow>, Box<dyn Error>> {
    let mut spring_rows: Vec<SpringsRow> = Vec::new();
    for (line_index, line) in file_content.lines().enumerate(){
        let trimmed = line.trim();
        let mut row_string: Option<&str> = None;
        let mut arrangement_string: Option<&str> = None;
        for (snippet_index_in_line, snippet) in trimmed.split_whitespace().enumerate() {
            match snippet_index_in_line {
                0 => row_string = Some(snippet),
                1 => arrangement_string = Some(snippet),
                _ => return Err(MyError::new_as_box(&format!("Only 2 snippets were expected, see line: {}", line_index), None)),
            }
        }
        if let Some(row_data) = row_string {
            if let Some(arrangement_data) = arrangement_string {
                spring_rows.push(SpringsRow::new(row_data, arrangement_data)?);
            }
            else { return Err(MyError::new_as_box(&format!("Found row data, but no arrangement data on line {}", line_index), None));}
        }
        else { return Err(MyError::new_as_box(&format!("Could not find data on line {}", line_index), None));}
    }
    Ok(spring_rows)
}
