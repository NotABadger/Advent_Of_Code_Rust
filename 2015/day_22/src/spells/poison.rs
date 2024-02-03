use crate::effect_trait::Effect;

#[derive(Debug)]
pub struct Poison{
    name: String,
    lasts_turns: i32,
}

impl Effect for Poison { 
    //get name of spell
    fn get_name(&self) -> String {
        self.name.clone()
    }
    //get mana cost of the effect
    fn get_cost(&self) -> i32 {
        173
    }
    //get damage an effect gives
    fn get_dmg(&self) -> i32 {
        3
    }
}

impl Poison {
    const LASTS_TURNS: i32 = 3;
    const NAME: &str = "Poison";
    
    pub fn new() -> Self {
        Self{name: Self::NAME.to_string(), lasts_turns: Self::LASTS_TURNS}
    }
}