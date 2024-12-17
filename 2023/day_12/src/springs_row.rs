use error_type_lib::MyError;

use crate::spring_state::SpringState;
use std::error::Error;

#[derive(Debug)]
pub struct SpringsRow {
    original_row: Vec<SpringState>,
    arrangement_order: Vec<u8>,
    unknown_indexes: Vec<usize>,
}

impl SpringsRow {
    pub fn new(row_state: &str, arrangement_order: &str) -> Result<Self, Box<dyn Error>> {
        let mut converted_row_states: Vec<SpringState> = Vec::new();
        let mut converted_row_order: Vec<u8> = Vec::new();
        let mut scanned_unknown_indexes: Vec<usize> = Vec::new();
        //parsing
        for (str_index, state_char) in row_state.chars().enumerate() {
            match SpringState::try_from(state_char) {
                Ok(spring_state) => { converted_row_states.push(spring_state);
                                                   if spring_state == SpringState::Unknown {
                                                    scanned_unknown_indexes.push(str_index);
                                                   }},
                Err(err) => return Err(MyError::new_as_box(
                                            &format!("Parse {} to state", state_char), Some(err))),
            }
        }

        for arrangment_str in arrangement_order.split(',') {
            //should be 1 char strings  
            match arrangment_str.parse::<u8>(){
                Ok(number) => converted_row_order.push(number),
                Err(error) => return Err(MyError::new_as_box(
                    &format!("Parse {} to arrangment number", arrangment_str), Some(Box::new(error)))),
            }
        }
        Ok(Self { original_row: converted_row_states, arrangement_order: converted_row_order, unknown_indexes: scanned_unknown_indexes})
    }

    pub fn get_row_spring_states(&self) -> &Vec<SpringState> {
        &self.original_row
    }

    pub fn get_arrangment_order(&self) -> &Vec<u8> {
        &self.arrangement_order
    }

    pub fn get_unknown_springstate_indexes(&self) -> &Vec<usize> {
        &self.unknown_indexes
    }
}