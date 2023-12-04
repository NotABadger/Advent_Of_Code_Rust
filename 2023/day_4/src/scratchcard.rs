#[derive(Debug,Clone)]
pub struct Scratchcard {
    number: u32,
    winning_numbers: Vec<u32>,
    scratched_numbers: Vec<u32>,
    value: u32,
    amount_numbers_won: u32,
}

impl Scratchcard {
    pub fn new(id : u32) -> Self {
        Self { number: id, winning_numbers: Vec::new(), scratched_numbers: Vec::new(), value: 0 , amount_numbers_won: 0}
    }

    pub fn add_winning_number(&mut self, winning_number: u32) {
        self.winning_numbers.push(winning_number);
    }

    pub fn add_scratched_number(&mut self, scratched_number: u32) {
        self.scratched_numbers.push(scratched_number);
    }

    pub fn get_value(&self) -> u32 {
        self.value
    }

    pub fn get_number(&self) -> u32 {
        self.number
    }

    pub fn get_amount_numbers_won(&self) -> u32 {
        self.amount_numbers_won
    }

    pub fn determine_value(&mut self){
        for sc_number in &self.scratched_numbers
        {
            if self.winning_numbers.contains(sc_number)
            {
                if self.value == 0
                {
                    self.value = 1;
                }
                else {
                    self.value = self.value * 2;
                }
                self.amount_numbers_won += 1;
            }
        }
    }
}