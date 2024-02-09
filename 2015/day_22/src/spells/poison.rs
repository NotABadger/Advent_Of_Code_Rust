use crate::effect_trait::Effect;

#[derive(Debug, Clone)]
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
    //deduct rounds
    fn deduct_rounds_active(&mut self) {
        self.lasts_turns -= 1;
    }
    //Copy all values
    fn deep_copy_effect(&self) -> Box<dyn Effect>
    {
        Box::new(Poison::new())
    }
}

impl Poison {
    const LASTS_TURNS: i32 = 3;
    const NAME: &str = "Poison";
    
    pub fn new() -> Self {
        Self{name: Self::NAME.to_string(), lasts_turns: Self::LASTS_TURNS}
    }
}