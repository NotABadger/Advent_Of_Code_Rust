use crate::game_round::GameRound;

#[derive(Debug)]
pub struct Game{
    id: u32,
    pub rounds: Vec<GameRound>,
}

impl Default for Game{
    fn default() -> Self {
        Game { id: 0, rounds: Vec::new() }
    }
}

impl Game {
    pub fn new(game_id: u32) -> Self {
        Game { id: game_id, rounds: Vec::new() }
    }

    pub fn get_id(&self) -> u32 {
        self.id
    }

    pub fn add_round(&mut self, round: GameRound)
    {
        self.rounds.push(round);
    }
}