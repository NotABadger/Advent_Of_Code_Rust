use crate::{game::Game, game_round::GameRound};

pub fn parse_games_from_text(text: &str) -> Vec<Game> {
    
    let mut ret_game: Vec<Game> = Vec::new();
    
    for line in text.lines()
    {
        let game_substr: &str = &line[5..line.find(':').unwrap()];
        let mut game: Game = Game::new(game_substr.trim().parse::<u32>().unwrap());
        let rounds_substr: &str = &line[line.find(':').unwrap()+1..];
        for round in rounds_substr.split(';')
        {
            let mut game_round: GameRound = GameRound::default();
            for color_str in round.split(',')
            {
                let trimmed_str: &str = color_str.trim();
                let number_str: &str = &trimmed_str[0..trimmed_str.find(' ').unwrap()];
                let number: u32 = number_str.parse::<u32>().unwrap();
                let color_str: &str = &trimmed_str[trimmed_str.find(' ').unwrap()..];
                match color_str.trim()
                {
                    "red" => game_round.red = number,
                    "green" => game_round.green = number,
                    "blue" => game_round.blue = number,
                    _ => println!("got weird input in color string matching {}", color_str.trim()),
                }
            }
            game.add_round(game_round)
        }
        ret_game.push(game);
    }

    ret_game
}