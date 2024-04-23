pub use crate::card::Card;
use crate::card_combinations::{Combination, get_combination};

use std::cmp::Ordering;

#[derive(Debug, Eq, Ord)]
pub struct HandOfCards {
    pub cards: [Card; 5],
    pub bid: u32, 
    pub best_combination: Option<Combination>,
}

impl From<&str> for HandOfCards {
    fn from(value: &str) -> Self {
        let mut ret_val: HandOfCards =  Self::default();
        let trimmed_ref = value.trim(); //a line of the input file
        let mut words_itt = trimmed_ref.split_whitespace();
        let cards = words_itt.next().expect("Expected cards symbold");
        for card_symbol in cards.chars().enumerate() {
            ret_val.cards[card_symbol.0] = Card::from(card_symbol.1);
        }
        ret_val.bid = words_itt.next().expect("Expected bid value").trim().parse::<u32>().expect("Expected bid ");
        ret_val.best_combination = Some(get_combination(&ret_val.cards));
        ret_val
    }
}

impl Default for HandOfCards {
    fn default() -> Self {
        Self { cards: [Card::TWO; 5], bid: 0, best_combination: None }
    }
}

impl PartialEq for HandOfCards {
    fn eq(&self, other: &Self) -> bool {
        if self.best_combination.is_none() && other.best_combination.is_none() {
            return true;
        }
        if self.best_combination == other.best_combination {
            for index in 0.. self.cards.len() {
                if self.cards[index] != other.cards[index]
                {
                    return false;
                }
            }
            return true;
        }
        false
    }
}

impl PartialOrd for HandOfCards {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.eq(other) {
            return Some(Ordering::Equal);
        }
        if self.best_combination.is_none() && other.best_combination.is_some() {
            return Some(Ordering::Less);
        }
        if self.best_combination.is_some() && other.best_combination.is_none() {
            return Some(Ordering::Greater);
        }
        if self.best_combination > other.best_combination {
            return Some(Ordering::Greater);
        }
        if self.best_combination < other.best_combination {
            return Some(Ordering::Less);
        }
        //best_combination are equal, so cards make the difference.
        for index in 0.. self.cards.len() {
            if self.cards[index] > other.cards[index]
            {
                return Some(Ordering::Greater);
            }
            if self.cards[index] < other.cards[index]
            {
                return Some(Ordering::Less);
            }
        }
        return Some(Ordering::Equal);
    }
}

