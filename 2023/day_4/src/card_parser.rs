use crate::scratchcard::Scratchcard;

pub fn parse_input_to_scratchcards(input_content: &str) -> Vec<Scratchcard>
{
    let mut return_list: Vec<Scratchcard> = Vec::new();
    for line in input_content.lines()
    {
        let mut current_scratch: Scratchcard = Scratchcard::new(0);
        for (index, word) in line.split_whitespace().enumerate()
        {
            match index {
                1 => {
                    let mut id_str = word.trim();
                    id_str= id_str.trim_end_matches(':');
                    current_scratch = Scratchcard::new(id_str.parse::<u32>().expect("Expected id number"));
                }
                2..=11 => { 
                    let number: u32 = word.parse::<u32>().expect("expected winning number here");
                    current_scratch.add_winning_number(number);
                }
                //12 skiped, is seperator
                13..=37 => {
                    let number: u32 = word.parse::<u32>().expect("expected winning number here");
                    current_scratch.add_scratched_number(number);
                },
                _ => (),
            }
        }
        assert_ne!(current_scratch.get_number(), 0, "scratch cars that has no initialized index");
        current_scratch.determine_value();
        return_list.push(current_scratch);
    }

    return_list
}