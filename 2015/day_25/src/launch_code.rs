

pub struct LaunchCode {
    column: u32,
    row: u32,
    code: u32,
}

impl LaunchCode {
    pub fn new(column: u32, row: u32, code: u32) -> Self {
        LaunchCode { column, row, code }
    }

    pub fn generate_from_previous(prev_code: &Self, highest_row: &mut u32) -> Self {
        //for hard coded numbers, see challange
        let mut new_code_val: u64 = prev_code.code as u64 * 252533 as u64;
        new_code_val = new_code_val % 33554393 as u64;
       
        //
        let mut new_column: u32 = prev_code.get_column() + 1;
        let new_row: u32;
        if prev_code.get_row() == 1 {
            *highest_row += 1;
            new_row = *highest_row;
            new_column = 1;
        }
        else {
            new_row = prev_code.get_row() - 1;
        }
        LaunchCode::new(new_column, new_row, new_code_val as u32)
    }

    pub fn get_row(&self) -> u32 {
        self.row
    }

    pub fn get_column(&self) -> u32 {
        self.column
    }

    pub fn get_code(&self) -> u32 {
        self.code
    }
}

impl Default for LaunchCode {
    fn default() -> Self {
        Self::new(1, 1, 0)
    }
}