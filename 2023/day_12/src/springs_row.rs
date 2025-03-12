use error_type_lib::MyError;

use crate::spring_state::SpringState;
use std::error::Error;

#[derive(Debug, Clone)]
pub struct SpringsRow {
    springs_in_row: Vec<SpringState>,
    arrangement_broken_springs: Vec<u8>,
    unknown_state_indexes: Vec<usize>,
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
        Ok(Self { springs_in_row: converted_row_states, arrangement_broken_springs: converted_row_order, unknown_state_indexes: scanned_unknown_indexes})
    }

    pub fn get_row_spring_states(&self) -> &Vec<SpringState> {
        &self.springs_in_row
    }

    pub fn get_arrangment_order(&self) -> &Vec<u8> {
        &self.arrangement_broken_springs
    }

    pub fn get_unknown_springstate_indexes(&self) -> &Vec<usize> {
        &self.unknown_state_indexes
    }

    pub fn verify_row_correctness_self(&self) -> bool {
        SpringsRow::verify_row_correctness(&self.springs_in_row, &self.arrangement_broken_springs)
    }

    pub fn verify_row_correctness(row_of_springs: &Vec<SpringState>, arrangement_order: &Vec::<u8> ) -> bool {
        //this arrangement should not have questionmarks
        if row_of_springs.contains(&SpringState::Unknown) {
            return false;
        }
        let mut found_arrangment_order = Vec::<u8>::new();
        let mut current_count: u8 = 0;
        let mut last_spring_state = SpringState::Operational;
        for springs in row_of_springs {
            //not counting, counting, not counting
            if *springs == SpringState::Operational {
                if last_spring_state == SpringState::Broken {
                    if current_count > 0 {
                        found_arrangment_order.push(current_count);
                        current_count = 0;
                    }
                }
            }
            if *springs == SpringState::Broken {
                current_count += 1;
            }
            last_spring_state = *springs;
        }
        //in case last spring was broken
        if current_count > 0 {
            found_arrangment_order.push(current_count);
        }
        arrangement_order.eq(&found_arrangment_order)
    }
}