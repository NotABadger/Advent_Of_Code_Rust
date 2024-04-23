use std::collections::BTreeMap;

use crate::card::Card;

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord)]
pub enum Combination {
    FiveOfKind = 1,
    FourOfKind = 2,
    FullHouse = 3,
    ThreeOfKind = 4,
    TwoPair = 5,
    OnePair = 6,
    HighCard = 7,
}

pub fn get_combination(cards: &[Card; 5]) -> Combination {
    //kind of cards? amount of different ones?
    let mut kind_of_cards: BTreeMap<Card, u8> = BTreeMap::new();
    for card in cards {
        match kind_of_cards.get_mut(card) {
            Some(found) => *found += 1,
            None => _ = kind_of_cards.insert(card.clone(), 1),
        }
    }
    //find combination
    let mut found_pair: u8 = 0;
    let mut found_triple: bool = false;

    for (_card_type, amount) in &kind_of_cards {
        if *amount == 5 {
            return Combination::FiveOfKind;
        }
        if *amount == 4 {
            return Combination::FourOfKind;
        }
        if *amount == 3 {
            found_triple = true;
        }
        if *amount == 2 {
            found_pair += 1;
        } 
    }
    if found_triple && found_pair == 1 {
        //full house
        return Combination::FullHouse;
    }

    if found_triple {
        return Combination::ThreeOfKind;

    }

    if found_pair == 2 {
        // found 2 pair
        return Combination::TwoPair;
    }

    if found_pair == 1 {
        return Combination::OnePair;

    }
    //highest card
    Combination::HighCard
}