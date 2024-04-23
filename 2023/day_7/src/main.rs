extern crate input_file_lib;
mod hand_of_cards;
mod card;
mod card_combinations;

use input_file_lib::read_file_content;
use crate::hand_of_cards::HandOfCards;

fn main() {
    println!("Day 7 program: ");
    let file_cont: String = read_file_content().expect("Program expects an input file as parameter");
    let mut list_of_hands: Vec<HandOfCards> = Vec::with_capacity(file_cont.lines().count());
    file_cont.lines().for_each(|line| list_of_hands.push(HandOfCards::from(line)));
    list_of_hands.sort();
    list_of_hands.reverse(); //lowest val is on top
    let mut total_winnigs: u64 = 0;
    for (index, hand) in list_of_hands.iter().enumerate() {
        let hand_score = (index as u64 +1) * hand.bid as u64;
        total_winnigs += hand_score;
    }
    
    println!("Total winnings: {}", total_winnigs);
}