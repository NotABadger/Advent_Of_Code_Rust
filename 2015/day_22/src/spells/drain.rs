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
        73
    }
    //get damage an effect does
    fn get_dmg(&self) -> i32 {
        2
    }
    //get healing an effect does
    fn get_healing(&self) -> i32 {
        2
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

    pub fn new() -> Self {
        Self{}
    }
}