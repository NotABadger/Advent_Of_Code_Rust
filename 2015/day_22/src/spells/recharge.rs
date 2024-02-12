use crate::effect_trait::Effect;

#[derive(Debug, Clone)]
pub struct Recharge{
    lasts_turns: i32,
}

impl Effect for Recharge { 
    //get name of spell
    fn get_name(&self) -> String {
        Self::NAME.to_string()
    }
    //get mana cost of the effect
    fn get_cost(&self) -> i32 {
        229
    }
    //get mana an effect gives
    fn get_mana(&self) -> i32 {
        101
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
        Box::new(Recharge::new())
    }
}

impl Recharge {
    pub const NAME: &'static str = "Recharge";
    const LASTS_TURNS: i32 = 5;
    
    pub fn new() -> Self {
        Self{lasts_turns: Self::LASTS_TURNS}
    }
}