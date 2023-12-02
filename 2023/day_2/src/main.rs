mod game;
mod game_round;
mod games_parser;

use input_file_lib::read_file_content;
use games_parser::parse_games_from_text;

fn main() {
    let input_content: String = read_file_content().unwrap();
    let games_list: Vec<game::Game> = parse_games_from_text(&input_content);

    //the bag only contains: //PART 1
    let max_red: u32 = 12;
    let max_green: u32 = 13;
    let max_blue: u32 = 14;
    let mut total_of_possible_ids: u32 = 0;

    //power of minimum required dice PART 2 
    let mut power_total_min_req_cubes: u32 = 0;

    //go through all games, and check which ones are possible
    for game in &games_list
    {
        let mut impossible: bool = false;

        //the minimum a bag should contain // PART 2
        let mut min_red: u32 = 0;
        let mut min_green: u32 = 0;
        let mut min_blue: u32 = 0;

        for round in &game.rounds
        {
            if round.red > max_red
            {
                impossible = true;
            }
            if round.green > max_green
            {
                impossible = true;
            }
            if round.blue > max_blue
            {
                impossible = true;
            }
            //part 2 checks
            if round.red > min_red
            {
                min_red = round.red
            }
            if round.green > min_green
            {
                min_green = round.green
            }
            if round.blue > min_blue
            {
                min_blue = round.blue
            }
        }

        power_total_min_req_cubes += min_red * min_blue * min_green; // part 2

        if impossible
        {
            continue;
        }
        total_of_possible_ids += game.get_id();
    }

    println!("total ID of possible games: {}", total_of_possible_ids);
    println!("total combined power if minimal required cubes is: {}", power_total_min_req_cubes);
}
