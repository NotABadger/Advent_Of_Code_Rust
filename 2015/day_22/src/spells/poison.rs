use crate::effect_trait::Effect;

#[derive(Debug, Clone)]
pub struct Poison{
    lasts_turns: i32,
}

impl Effect for Poison { 
    //get name of spell
    fn get_name(&self) -> String {
        Self::NAME.to_string()
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
    //Check how many rounds this effect is active
    fn get_rounds_active(&self) -> i32 {
        self.lasts_turns
    }
    //Copy all values
    fn deep_copy_effect(&self) -> Box<dyn Effect>
    {
        Box::new(Poison::new())
    }
}

impl Poison {
    pub const NAME: &'static str = "Poison";
    const LASTS_TURNS: i32 = 3;
    
    pub fn new() -> Self {
        Self{lasts_turns: Self::LASTS_TURNS}
    }
}