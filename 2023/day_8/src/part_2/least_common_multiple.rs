extern crate num;
use num::integer::lcm;

pub fn find_least_common_multiplicator(mut numbers: Vec<u32>) -> u64{
    numbers.sort();
    numbers.reverse();

    let mut common_multiplier: u64 = 1;  
    for (index, number) in numbers.iter().enumerate() {
        if index == 0 {
            common_multiplier = *number as u64
        }
        else {
            common_multiplier = lcm(common_multiplier, *number as u64);
        }
    }

    common_multiplier
}