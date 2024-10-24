
#[derive(Debug)]
pub struct OasisReport {
    report: Vec<Vec<i32>>,
    future_extrapolation: i32,
    past_extrapolation: i32,
}

impl OasisReport {
    pub fn new(start_data: Vec<Vec<i32>>) -> Self {
        Self { report: start_data, future_extrapolation: 0, past_extrapolation: 0 }
    }

    pub fn part1_prediction(&mut self) -> i32 {
        self.analyze_report();
        
        //predict, grab last of previous line, add up, all up to line 0
        self.future_extrapolation= 0;
        for line in self.report.iter_mut().rev() {
            let last_number = *line.last().unwrap();
            self.future_extrapolation = last_number + self.future_extrapolation;
            //Stephan Broekman
            //Joris Peijnacker 
        }
        self.future_extrapolation
    }


    pub fn part2_prediction(&mut self) -> i32 {
        if self.report.len() < 2 {
            self.analyze_report();
        }
        //predict, grab last of previous line, add up, all up to line 0
        self.past_extrapolation= 0;
        for line in self.report.iter().rev() {
            let first_number = *line.first().unwrap();
            self.past_extrapolation = first_number - self.past_extrapolation;
        }
        self.past_extrapolation
    }

    fn analyze_report(&mut self) {
         //check differences, and add data until all 0's 
         let mut all_zeros: bool = false;
         while !all_zeros {
             let mut next_line: Vec<i32> = Vec::new();
             let current_line = self.report.last().unwrap();
             //create 2 iterators, one on index 0, one on index 1
             let mut iter_on_ind_one = current_line.into_iter();
             match iter_on_ind_one.next() {
                 Some(_) => (),
                 None => break, // no numbers available
             }
             let zip_iter = current_line.into_iter().zip(iter_on_ind_one);
             for (index_current, index_next) in zip_iter {
                 next_line.push(index_next - index_current);
             }
             all_zeros = true;
             if next_line.len() > 0 {
                 for number in &next_line {
                     if *number != 0 {
                         all_zeros = false;
                         break;
                     }
                 }
                 self.report.push(next_line);
             }
         }
    }
}