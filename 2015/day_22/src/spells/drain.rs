use crate::effect_trait::Effect;

#[derive(Debug, Clone)]
pub struct Drain {
    name: String,
}

impl Effect for Drain {
    //get name of spell
    fn get_name(&self) -> String {
        self.name.clone()
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
    const NAME: &str = "Drain";

    pub fn new() -> Self {
        Self{name: Self::NAME.to_string()}
    }
}