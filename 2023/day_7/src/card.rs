use std::convert::From;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Card {
    ACE = 1,
    KING = 2,
    QUEEN = 3,
    JACK = 4,
    TEN = 5,
    NINE = 6,
    EIGHT = 7,
    SEVEN = 8,
    SIX = 9,
    FIVE = 10,
    FOUR = 11,
    THREE = 12,
    TWO = 13,
}
impl From<&str> for Card {
    fn from(value: &str) -> Self {
        match value.trim() {
            "2" => Card::TWO,
            "3" => Card::THREE,
            "4" => Card::FOUR,
            "5" => Card::FIVE,
            "6" => Card::SIX,
            "7" => Card::SEVEN,
            "8" => Card::EIGHT,
            "9" => Card::NINE,
            "T" => Card::TEN,
            "J" => Card::JACK,
            "Q" => Card::QUEEN,
            "K" => Card::KING,
            "A" => Card::ACE,
            _ => panic!("Parsing card failed"),
        }
    }
}

impl From<char> for Card {
    fn from(value: char) -> Self {
        match value {
            '2' => Card::TWO,
            '3' => Card::THREE,
            '4' => Card::FOUR,
            '5' => Card::FIVE,
            '6' => Card::SIX,
            '7' => Card::SEVEN,
            '8' => Card::EIGHT,
            '9' => Card::NINE,
            'T' => Card::TEN,
            'J' => Card::JACK, 
            'Q' => Card::QUEEN,
            'K' => Card::KING,
            'A' => Card::ACE,
            _ => panic!("Parsing card failed"),
        }
    }
}

