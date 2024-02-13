use crate::effect_trait::Effect;

#[derive(Debug, Clone)]
pub struct Drain {
}

impl Effect for Drain {
    //get name of spell
    fn get_name(&self) -> String {
        Self::NAME.to_string()
    }
    //get mana cost of the effect
    fn get_cost(&self) -> i32 {
        Self::COST
    }
    //get damage an effect does
    fn get_dmg(&self) -> i32 {
        Self::DMG
    }
    //get healing an effect does
    fn get_healing(&self) -> i32 {
        Self::HEAL
    }
    //deduct rounds, but has no active rounds
    fn deduct_rounds_active(&mut self) {
        
    }
    //Copy all values
    fn deep_copy_effect(&self) -> Box<dyn Effect>
    {
        Box::new(Drain::new())
    }
}

impl Drain {
    pub const NAME: &'static str = "Drain";
    const COST: i32 = 73;
    const DMG: i32 = 2;
    const HEAL: i32 = 2;
    

    pub fn new() -> Self {
        Self{}
    }
}