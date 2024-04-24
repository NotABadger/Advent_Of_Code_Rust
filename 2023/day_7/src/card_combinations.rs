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
            None => _ = kind_of_cards.insert(*card, 1),
        }
    }
    //find combination
    let mut found_pair: u8 = 0;
    let mut found_triple: bool = false;
    let mut found_quadrupple: bool = false;

    for (_card_type, amount) in &kind_of_cards {
        if *amount == 5 {
            return Combination::FiveOfKind;
        }
        if *amount == 4 {
            found_quadrupple = true;
        }
        if *amount == 3 {
            found_triple = true;
        }
        if *amount == 2 {
            found_pair += 1;
        } 
    }

    if found_quadrupple {
        //found four of a kind
        if kind_of_cards.contains_key(&Card::JACK) {
            return Combination::FiveOfKind;
        }
        return Combination::FourOfKind;
    }
    if found_triple && found_pair == 1 {
        //full house
        if kind_of_cards.contains_key(&Card::JACK) {
            //full house counts all 5 cards, so if either is a Jack, it is 5 of kind
            return Combination::FiveOfKind;
        }
        return Combination::FullHouse;
    }

    if found_triple {
        //found a triple, might have jacks
        if let Some(val_ref) = kind_of_cards.get(&Card::JACK) {
            //triple, -> either tripple + jack(s), or triple == jacks
            if *val_ref == 1 || *val_ref == 3 {
                return Combination::FourOfKind;
            }
            if *val_ref == 2 {
                return Combination::FiveOfKind;
            }
        }
        return Combination::ThreeOfKind;
    }

    if found_pair == 2 {
        // found 2 pair
        if let Some(val_ref) = kind_of_cards.get(&Card::JACK) {
            //1 jack, and 2 pairs of two, makes full house
            if *val_ref == 1 {
                return Combination::FullHouse;
            }
            if *val_ref == 2 { //one of the pairs is a pair of jacks
                return Combination::FourOfKind;
            }
        }
        return Combination::TwoPair;
    }

    if found_pair == 1 {
       // found 1 pair, so either pair of jacks, or pair + 1 jack
       if kind_of_cards.contains_key(&Card::JACK) {
            return Combination::ThreeOfKind;
        }
        return Combination::OnePair;
    }
    //highest card, with optional jack
    if kind_of_cards.contains_key(&Card::JACK) {
        return Combination::OnePair;
    }
    Combination::HighCard
}