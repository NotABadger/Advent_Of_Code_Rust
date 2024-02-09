use crate::effect_trait::Effect;

#[derive(Debug, Clone)]
pub struct Shield{
    name: String,
    lasts_turns: i32,
}

impl Effect for Shield { 
    //get name of spell
    fn get_name(&self) -> String {
        self.name.clone()
    }
    //get mana cost of the effect
    fn get_cost(&self) -> i32 {
        113
    }
    //get armor an effect gives
    fn get_armor(&self) -> i32 {
        7
    }
    //deduct rounds
    fn deduct_rounds_active(&mut self) {
        self.lasts_turns -= 1;
    }
    //Copy all values
    fn deep_copy_effect(&self) -> Box<dyn Effect>
    {
        Box::new(Shield::new())
    }
}

impl Shield {
    const NAME: &str = "Shield";
    const LASTS_TURNS: i32 = 6;
    
    pub fn new() -> Self {
        Self{name: Self::NAME.to_string(), lasts_turns: Self::LASTS_TURNS}
    }
}