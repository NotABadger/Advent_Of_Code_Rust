use crate::effect_trait::Effect;

#[derive(Debug)]
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
}

impl Drain {
    const NAME: &str = "Drain";

    pub fn new() -> Self {
        Self{name: Self::NAME.to_string()}
    }
}