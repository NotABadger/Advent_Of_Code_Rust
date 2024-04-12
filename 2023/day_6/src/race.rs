pub struct Race {
    total_time: u64,
    current_record: u64,
}

impl Race {
    pub fn new(time: u64, current_record: u64) ->Self {
        Race{total_time: time, current_record}
    }

        //in how many ways can we beat it? 
    pub fn different_ways_of_winning(&self) -> u64 {
        let mut times_won: u64 = 0;
        for time_btn_held in 0..=self.total_time {
            let speed: u64 = time_btn_held;
            let remaining_time: u64 = self.total_time - time_btn_held;
            if remaining_time * speed > self.current_record {
                times_won += 1;
            }
        }
        times_won
    }
    pub fn different_ways_of_winning_long_race(&self) -> u64 {
       self.check_higher_values() + self.check_lower_values()
    }

    fn check_lower_values(&self) -> u64 {
        let mut times_won: u64 = 0;
        let mut race_won: bool = true;
        let mut time_btn_held: u64 = self.total_time / 2;
        
        while race_won
        {
            let remaining_time: u64 = self.total_time - time_btn_held;
            let speed: u64 = time_btn_held;
            if speed * remaining_time > self.current_record {
                times_won += 1;
                time_btn_held -= 1;
            }
            else {
                race_won = false;
            }
        }
        times_won
    }

    fn check_higher_values(&self) -> u64 {
        let mut times_won: u64 = 0;
        let mut race_won: bool = true;
        let mut time_btn_held: u64 = (self.total_time / 2) + 1;
        
        while race_won
        {
            let remaining_time: u64 = self.total_time - time_btn_held;
            let speed: u64 = time_btn_held;
            if speed * remaining_time > self.current_record {
                times_won += 1;
                time_btn_held += 1;
            }
            else {
                race_won = false;
            }
        }
        times_won
    }


}