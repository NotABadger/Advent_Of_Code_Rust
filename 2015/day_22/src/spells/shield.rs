use crate::effect_trait::Effect;

#[derive(Debug, Clone)]
pub struct Shield{
    lasts_turns: i32,
}

impl Effect for Shield { 
    //get name of spell
    fn get_name(&self) -> String {
        Self::NAME.to_string()
    }
    //get mana cost of the effect
    fn get_cost(&self) -> i32 {
        Self::COST
    }
    //get armor an effect gives
    fn get_armor(&self) -> i32 {
        Self::ARMOR
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
        let mut cpy: Self = Shield::new();
        cpy.lasts_turns = self.lasts_turns;
        Box::new(cpy)
    }
}

impl Shield {
    pub const NAME: &'static str = "Shield";
    const COST: i32 = 113;
    const LASTS_TURNS: i32 = 6;
    const ARMOR: i32 = 7;
    
    pub fn new() -> Self {
        Self{lasts_turns: Self::LASTS_TURNS}
    }
}