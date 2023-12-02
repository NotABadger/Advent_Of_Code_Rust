
#[derive(Debug)]
pub struct GameRound{
    pub red: u32,
    pub green: u32,
    pub blue: u32,
}

impl Default for GameRound {
    fn default() -> Self {
        GameRound { red: 0, green: 0, blue: 0 }
    }
}