mod oasis_report;
use input_file_lib::read_file_content;
use threadpool_lib::initialize_threadpool;

use oasis_report::OasisReport;

fn main() {
    println!("Day 9 program");
    let file_cont: String = read_file_content().expect("Program expects an input file as parameter");
    let mut data_list: Vec<OasisReport> = Vec::new();
    //input parsing
    for line in file_cont.lines() {
        let trimmed = line.trim();
        let mut data_sample: Vec<i32> = Vec::new();
        for number in trimmed.split_whitespace() {
            data_sample.push(number.parse::<i32>().unwrap());
        }
        data_list.push(OasisReport::new(vec![data_sample]));
    }
    
    //let pool = initialize_threadpool();
    let mut sum_future_predictions: i32 = 0;
    let mut sum_past_predictions: i32 = 0;
    for data in &mut data_list {
        //pool.execute
        sum_future_predictions += data.part1_prediction();
        sum_past_predictions += data.part2_prediction();
    }
    println!("Sum of all future predictions: {}", sum_future_predictions);
    println!("Sum of all past predictions: {}", sum_past_predictions);

}
