mod scratchcard;
mod card_parser;

use input_file_lib::read_file_content;
fn main() {
    let file_content: String = read_file_content().expect("Expected input file as parameter");
    let mut list_of_cards: Vec<scratchcard::Scratchcard> = card_parser::parse_input_to_scratchcards(&file_content);

    let mut total_value: u32 = 0;

    for card in &list_of_cards
    {
        total_value += card.get_value();
    }

    println!("we got {} cards", list_of_cards.len());
    println!("total value of cards: {}", total_value);

    // Part 2 \\

    let mut index: usize = 0;
    let mut loop_should_continue: bool = true;
    while loop_should_continue
    {
        let card = &list_of_cards[index];
        if card.get_amount_numbers_won() > 0
        {
            let cardnr_start: u32 = card.get_number() + 1;
            let cardnr_end: u32 = card.get_number() + card.get_amount_numbers_won();
            for cardnr_index in cardnr_start..=cardnr_end
            {
                list_of_cards.push(
                            list_of_cards.iter().find(|card| card.get_number() == cardnr_index)
                            .unwrap()
                            .clone());
            }

        }
        index += 1;
        if list_of_cards.len() == index
        {
            loop_should_continue = false;
        }
    }

    
    println!("You will have {} cards in total", list_of_cards.len());

}